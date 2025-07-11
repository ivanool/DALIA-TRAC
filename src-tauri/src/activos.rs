use tauri::command;
use crate::get_data;
use crate::get_data::EmisoraBusqueda;
use postgres::{Client, Error};
use chrono::{Datelike, NaiveDate, Local, Duration};
use chrono::NaiveDateTime;
use std::collections::HashMap;


#[command]
pub fn get_emisora_query(query: String) -> Result<Vec<EmisoraBusqueda>, String> {
    use postgres::{Client, NoTls};
    let mut client = Client::connect(
        "host=localhost user=garden_admin password=password dbname=dalia_db",
        NoTls,
    ).map_err(|e| e.to_string())?;
    let sql = r#"
        SELECT razon_social, emisoras, serie
        FROM emisoras
        WHERE (LOWER(razon_social) LIKE $1 OR LOWER(emisoras) LIKE $1)
          AND estatus = 'ACTIVA'
        ORDER BY razon_social
        LIMIT 20
    "#;
    let pattern = format!("%{}%", query.to_lowercase());
    let rows = client.query(sql, &[&pattern]).map_err(|e| e.to_string())?;
    let results = rows
        .into_iter()
        .map(|row| EmisoraBusqueda {
            razon_social: row.get(0),
            emisoras: row.get(1),
            serie: row.get(2),
        })
        .collect();
    Ok(results)
}

pub fn list_dividendos_by_activo(
    pg_client: &mut Client,
    portafolio_ticker_id: i32,
) -> Result<Vec<(i32, f64, chrono::NaiveDateTime)>, Box<dyn std::error::Error>> {
    let rows = pg_client.query(
        "SELECT id, monto, fecha FROM dividendos WHERE portafolio_ticker_id = $1 ORDER BY fecha DESC",
        &[&portafolio_ticker_id],
    )?;
    let mut dividendos = Vec::new();
    for row in rows {
        let id: i32 = row.get(0);
        let monto: f64 = row.get(1);
        let fecha: chrono::NaiveDateTime = row.get(2);
        dividendos.push((id, monto, fecha));
    }
    Ok(dividendos)
}

/// Retrieves the financial cash flow data for a given company (`emisora`) and quarter.
///
/// If the data is not available in the database, it attempts to fetch it using
/// `get_data::get_flujos_financieros`.
///
/// # Arguments
///
/// * `pg_client` - A mutable reference to an active PostgreSQL database connection.
/// * `emisora` - The name of the company (case-insensitive).
/// * `trimestre` - The quarter to query, e.g., `"2024Q4"`.
///
/// # Returns
///
/// A `HashMap<String, f64>` where each key represents a financial flow category
/// (such as `"flujo_operacion"`, `"utilidad_neta"`, etc.) and the corresponding value
/// is the associated amount.
///
/// If a column is `NULL`, it will default to `0.0`.
///
/// # Errors
///
/// Returns a `Box<dyn std::error::Error>` if any error occurs while querying the database
/// or fetching external data.
///
/// # Example
///
/// ```rust,no_run
/// let mut client = connect_to_db()?; // Assume this is implemented
/// let cash_flow = get_finantial_flow(&mut client, "WALMEX", "2024Q1")?;
/// println!("Operating cash flow: {}", cash_flow["flujo_operacion"]);
/// ```
pub fn get_finantial_flow(
    pg_client: &mut Client, 
    emisora: &str, 
    trimestre: &str
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {

    // 1. Verificar existencia
    let row = pg_client.query_one(
        "SELECT EXISTS (
            SELECT 1 FROM public.estado_flujos
            WHERE LOWER(emisora) = LOWER($1)
              AND LOWER(trimestre) = LOWER($2)
        ) AS existe",
        &[&emisora, &trimestre],
    )?;
    let existe: bool = row.get("existe");
    if !existe {
        get_data::get_flujos_financieros(pg_client, emisora, trimestre)?;
    }

    // 2. Solicitar los datos del flujo financiero
    let row = pg_client.query_one(
        "SELECT flujo_operacion, utilidad_neta, depreciacion, cambio_inventarios,
            cambio_cxc, cambio_cxp, impuestos_pagados, intereses_pagados, flujo_inversion,
            capex, venta_activos, compra_intangibles, flujo_financiamiento, prestamos_obtenidos, 
            pago_deuda, dividendos_pagados, recompras, cambio_efectivo, efectivo_final, efecto_tc, deterioros,
            partidas_no_monetarias, costos_financieros
        FROM public.estado_flujos
        WHERE LOWER(emisora) = LOWER($1)
          AND LOWER(trimestre) = LOWER($2)
        LIMIT 1",
        &[&emisora, &trimestre],
    )?;

    // 3. Organizar en un HashMap
    let columnas = [
        "flujo_operacion", "utilidad_neta", "depreciacion", "cambio_inventarios",
        "cambio_cxc", "cambio_cxp", "impuestos_pagados", "intereses_pagados", "flujo_inversion",
        "capex", "venta_activos", "compra_intangibles", "flujo_financiamiento", "prestamos_obtenidos", 
        "pago_deuda", "dividendos_pagados", "recompras", "cambio_efectivo", "efectivo_final", "efecto_tc", "deterioros",
        "partidas_no_monetarias", "costos_financieros"
    ];
    let mut estados_financieros = HashMap::new();
    for (i, col) in columnas.iter().enumerate() {
        let valor: Option<f64> = row.get(i);
        estados_financieros.insert(col.to_string(), valor.unwrap_or(0.0));
    }

    Ok(estados_financieros)
}



pub fn get_finantial_position(
    pg_client: &mut Client, 
    emisora: &str,
    trimestre: &str
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {

    // 1. Verificar existencia
    let row = pg_client.query_one(
        "SELECT EXISTS (
            SELECT 1 FROM public.estado_posicion
            WHERE LOWER(emisora) = LOWER($1)
              AND LOWER(trimestre) = LOWER($2)
        ) AS existe",
        &[&emisora, &trimestre],
    )?;
    let existe: bool = row.get("existe");
    if !existe {
        get_data::get_posicion_financiera(pg_client, emisora, trimestre)?;
    }

    // 2. Solicitar los datos de la posición financiera
    let row = pg_client.query_one(
        "SELECT currentassets, currentliabilities, cashandcashequivalents, inventories,
            tradeandothercurrentreceivables, tradeandothercurrentpayables, equity, liabilities,
            noncurrentliabilities, equityattributabletoownersofparent, noncontrollinginterests,
            propertyplantandequipment, intangibleassetsotherthangoodwill, goodwill,
            rightofuseassetsthatdonotmeetdefinitionofinvestmentproperty, deferredtaxassets,
            deferredtaxliabilities, noncurrentassetsordisposalgroupsclassifiedasheldforsale,
            retainedearnings, issuedcapital, otherreserves, noncurrentleaseliabilities,
            othernoncurrentfinancialliabilities, noncurrentprovisionsforemployeebenefits
        FROM public.estado_posicion
        WHERE LOWER(emisora) = LOWER($1)
          AND LOWER(trimestre) = LOWER($2)
        LIMIT 1",
        &[&emisora, &trimestre],
    )?;

    // 3. Organizar en un HashMap
    let columnas = [
        "currentassets", "currentliabilities", "cashandcashequivalents", "inventories",
        "tradeandothercurrentreceivables", "tradeandothercurrentpayables", "equity", "liabilities",
        "noncurrentliabilities", "equityattributabletoownersofparent", "noncontrollinginterests",
        "propertyplantandequipment", "intangibleassetsotherthangoodwill", "goodwill",
        "rightofuseassetsthatdonotmeetdefinitionofinvestmentproperty", "deferredtaxassets",
        "deferredtaxliabilities", "noncurrentassetsordisposalgroupsclassifiedasheldforsale",
        "retainedearnings", "issuedcapital", "otherreserves", "noncurrentleaseliabilities",
        "othernoncurrentfinancialliabilities", "noncurrentprovisionsforemployeebenefits"
    ];
    let mut estados_posicion = HashMap::new();
    for (i, col) in columnas.iter().enumerate() {
        let valor: Option<f64> = row.get(i);
        estados_posicion.insert(col.to_string(), valor.unwrap_or(0.0));
    }

    Ok(estados_posicion)
}



pub fn get_quarterly_income_statement(
    pg_client: &mut Client,
    emisora: &str,
    trimestre: &str
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    // 1. Verificar existencia
    let row = pg_client.query_one(
        "SELECT EXISTS (
            SELECT 1 FROM public.estado_resultado_trimestral
            WHERE LOWER(emisora) = LOWER($1)
              AND LOWER(trimestre) = LOWER($2)
        ) AS existe",
        &[&emisora, &trimestre],
    )?;
    let existe: bool = row.get("existe");
    if !existe {
        get_data::get_estado_resultado_trimestral(pg_client, emisora, trimestre)?;
    }

    // 2. Solicitar los datos del estado de resultados trimesal
    // Ejecuta una consulta SQL para obtener una fila de la tabla `estado_resultado_trimestral`,
    // seleccionando varios campos financieros para una `emisora` y `trimestre` dados.
    // La consulta es case-insensitive para ambos parámetros y retorna campos como revenue,
    // gross profit, operating activities profit/loss, net profit/loss, cost of sales, expenses,
    // finance costs/income, tax expense, earnings per share, other income, share of profit/loss
    // from associates and joint ventures, discontinued operations profit/loss, y depreciación.
    let row = pg_client.query_one(
        "SELECT revenue, grossprofit, profitlossfromoperatingactivities, profitloss, profitlossbeforetax, \
            costofsales, distributioncosts, administrativeexpense, financecosts, financeincome, \
            incometaxexpensecontinuingoperations, profitlossattributabletoownersofparent, \
            basicearningslosspershare, dilutedearningslosspershare, otherincome, \
            shareofprofitlossofassociatesandjointventuresaccountedforusinge, \
            profitlossfromdiscontinuedoperations, depreciacion
        FROM public.estado_resultado_trimestral
        WHERE LOWER(emisora) = LOWER($1)
          AND LOWER(trimestre) = LOWER($2)
        LIMIT 1",
        &[&emisora, &trimestre],
    )?;

    // 3. Organizar en un HashMap
    let columnas = [
        "revenue", "grossprofit", "profitlossfromoperatingactivities", "profitloss", "profitlossbeforetax",
        "costofsales", "distributioncosts", "administrativeexpense", "financecosts", "financeincome",
        "incometaxexpensecontinuingoperations", "profitlossattributabletoownersofparent",
        "basicearningslosspershare", "dilutedearningslosspershare", "otherincome",
        "shareofprofitlossofassociatesandjointventuresaccountedforusinge",
        "profitlossfromdiscontinuedoperations", "depreciacion"
    ];
    let mut estado_resultado = HashMap::new();
    for (i, col) in columnas.iter().enumerate() {
        let valor: Option<f64> = row.get(i);
        estado_resultado.insert(col.to_string(), valor.unwrap_or(0.0));
    }

    Ok(estado_resultado)
}

#[tauri::command]
pub fn get_trimestres_disponibles(emisora: String) -> Result<Vec<String>, String> {
    use postgres::{Client, NoTls};
    let mut client = Client::connect(
        "host=localhost user=garden_admin password=password dbname=dalia_db",
        NoTls,
    ).map_err(|e| e.to_string())?;
    // Buscar en las tres tablas y unir los trimestres únicos
    let mut trimestres = std::collections::HashSet::new();
    for tabla in ["estado_flujos", "estado_posicion", "estado_resultado_trimestral"] {
        let sql = format!("SELECT DISTINCT trimestre FROM public.{} WHERE LOWER(emisora) = LOWER($1)", tabla);
        let rows = client.query(&sql, &[&emisora]).map_err(|e| e.to_string())?;
        for row in rows {
            let t: String = row.get(0);
            trimestres.insert(t);
        }
    }
    let mut trimestres_vec: Vec<String> = trimestres.into_iter().collect();
    trimestres_vec.sort();
    Ok(trimestres_vec)
}

#[tauri::command]
pub fn get_emisora_info(emisora: String, trimestre: Option<String>) -> Result<String, String> {
    use serde_json::json;
    let mut client = match postgres::Client::connect(
        "host=localhost user=garden_admin password=password dbname=dalia_db",
        postgres::NoTls,
    ) {
        Ok(c) => c,
        Err(e) => return Err(format!("DB connection error: {}", e)),
    };

    // Obtener los 4 trimestres más recientes si no se especifica
    let trimestres: Vec<String> = if let Some(t) = trimestre {
        vec![t]
    } else {
        let sql = "SELECT DISTINCT trimestre FROM public.estado_flujos WHERE LOWER(emisora) = LOWER($1) ORDER BY trimestre DESC LIMIT 4";
        let rows = client.query(sql, &[&emisora]).map_err(|e| e.to_string())?;
        let mut ts: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
        ts.sort();
        ts
    };

    let mut resultados = Vec::new();
    for t in &trimestres {
        let cashflow = match get_finantial_flow(&mut client, &emisora, t) {
            Ok(data) => data,
            Err(e) => return Err(format!("Error getting financial flow for {t}: {}", e)),
        };
        let position = match get_finantial_position(&mut client, &emisora, t) {
            Ok(data) => data,
            Err(e) => return Err(format!("Error getting financial position for {t}: {}", e)),
        };
        let income = match get_quarterly_income_statement(&mut client, &emisora, t) {
            Ok(data) => data,
            Err(e) => return Err(format!("Error getting income statement for {t}: {}", e)),
        };
        resultados.push(json!({
            "trimestre": t,
            "cashflow": cashflow,
            "position": position,
            "income": income
        }));
    }
    let result = json!({
        "emisora": emisora,
        "trimestres": trimestres,
        "datos": resultados
    });
    serde_json::to_string(&result).map_err(|e| format!("JSON serialization error: {}", e))
}

// --- NUEVO: Estructuras y comando para AssetViewPage ---
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IntradiaData {
    pub price: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub volume: i64,
    pub change: f64,
    pub change_percent: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinancialStatement {
    pub anio: i32,
    pub trimestre: String,
    pub utilidad_neta: f64,
    pub flujo_operativo: f64,
    pub depreciacion: f64,
    pub cambio_inventarios: f64,
    pub impuestos_pagados: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetDetails {
    pub razon_social: String,
    pub emisoras: String,
    pub serie: String,
    pub intradia: IntradiaData,
    pub quarterly_financials: Vec<FinancialStatement>,
}

#[tauri::command]
pub fn get_asset_details(ticker: String) -> Result<AssetDetails, String> {
    use postgres::{Client, NoTls};
    use chrono::{Local};
    let mut client = Client::connect(
        "host=localhost user=garden_admin password=password dbname=dalia_db",
        NoTls,
    ).map_err(|e| e.to_string())?;

    // 1. Info básica y API
    let row = client.query_one(
        "SELECT razon_social, emisoras, serie FROM emisoras WHERE emisoras = $1",
        &[&ticker],
    ).map_err(|e| format!("Error fetching basic info for {}: {}", ticker, e))?;
    let emisora_db: String = row.get("emisoras");
    let serie_db: String = row.get("serie");
    let ticker_key = format!("{}{}", emisora_db, serie_db);
    let cot_actual = crate::get_data::get_cotizaciones(&ticker_key).ok().flatten();
    let mut price = 0.0;
    let mut open = 0.0;
    let mut high = 0.0;
    let mut low = 0.0;
    let mut volume = 0;
    if let Some(cot) = &cot_actual {
        price = cot.ultimo_precio.unwrap_or(0.0);
        open = cot.precio_promedio.unwrap_or(0.0); // Usar precio_promedio como apertura
        // No hay campos high/low, los dejamos en 0.0
        volume = cot.volumen.unwrap_or(0.0) as i64;
    }

    // 2. Precio de cierre anterior (día hábil anterior)
    let previous_close_price_query = r#"
        SELECT precio
        FROM intradia_data
        WHERE emisora = $1 AND fecha_hora < current_date::timestamp
        ORDER BY fecha_hora DESC
        LIMIT 1;
    "#;
    let previous_close_price: f64 = match client.query_one(previous_close_price_query, &[&ticker_key]) {
        Ok(row) => row.get("precio"),
        Err(_) => {
            println!("[INFO] No se encontró cierre anterior para {}. Usando precio de apertura.", ticker_key);
            open
        }
    };
    if price == 0.0 && previous_close_price > 0.0 {
        price = previous_close_price;
    }
    let mut change = 0.0;
    let mut change_percent = 0.0;
    if previous_close_price > 0.0 {
        change = price - previous_close_price;
        change_percent = (change / previous_close_price) * 100.0;
    }
    let intradia = IntradiaData {
        price,
        open,
        high,
        low,
        volume,
        change,
        change_percent,
    };
    // 3. Últimos 4 trimestres financieros (sin cambios)
    let mut quarterly_financials = Vec::new();
    for row in client.query(
        "SELECT trimestre, flujo_operacion, utilidad_neta, depreciacion, cambio_inventarios, impuestos_pagados FROM estado_flujos WHERE emisora = $1 ORDER BY trimestre DESC LIMIT 4",
        &[&ticker],
    ).map_err(|e| format!("Error fetching financials: {}", e))? {
        let trimestre: String = row.get("trimestre");
        let anio = trimestre.get(0..4).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
        quarterly_financials.push(FinancialStatement {
            anio,
            trimestre: trimestre.clone(),
            utilidad_neta: row.get("utilidad_neta"),
            flujo_operativo: row.get("flujo_operacion"),
            depreciacion: row.get("depreciacion"),
            cambio_inventarios: row.get("cambio_inventarios"),
            impuestos_pagados: row.get("impuestos_pagados"),
        });
    }
    Ok(AssetDetails {
        razon_social: row.get("razon_social"),
        emisoras: row.get("emisoras"),
        serie: row.get("serie"),
        intradia,
        quarterly_financials,
    })
}
// --- FIN NUEVO ---
