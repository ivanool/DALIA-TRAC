use tauri::command;
use crate::get_data;
use crate::get_data::EmisoraBusqueda;
use postgres::{Client, Error};
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
