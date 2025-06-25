use chrono::{Local, Datelike, Timelike};
use postgres::Client;
use reqwest::blocking::Client as HttpClient;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use tauri::command;
use chrono::NaiveDate; 
use dotenv::dotenv;
use std::env;



#[derive(Debug, Serialize, Deserialize)]
pub struct Emisora {
    pub razon_social: String,
    pub isin: String,
    pub bolsa: String,
    #[serde(rename = "tipo_valor_descripcion")]
    pub tipo_valor: Option<String>,
    pub tipo_valor_id: String,
    pub estatus: String,
    #[serde(rename = "acciones_en_circulacion")]
    pub acciones_circulacion: Option<i64>,
    #[serde(rename = "rango_historicos")]
    pub rangos_historicos: Option<String>,
    #[serde(rename = "rango_financieros")]
    pub rangos_financieros: Option<String>,
    pub dividendos: Option<serde_json::Value>,
}

#[derive(Debug)]
pub struct Cotizacion {
    pub simbolo: String,
    pub ultimo_precio: Option<f64>,
    pub precio_promedio: Option<f64>,
    pub volumen: Option<f64>,
    pub fecha: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopImporte {
    pub e: String,
    pub i: f64,
    pub u: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopCambio {
    pub c: f64,
    pub e: String,
    pub f: String,
    pub u: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopOperaciones {
    pub e: String,
    pub o: i64,
    pub u: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopVolumen {
    pub e: String,
    pub i: f64,
    pub u: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopResponse {
    pub importe: Vec<TopImporte>,
    pub bajan: Vec<TopCambio>,
    pub operaciones: Vec<TopOperaciones>,
    pub suben: Vec<TopCambio>,
    pub volumen: Vec<TopVolumen>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForexResponse {
    pub t: String,
    pub USDMXN: Option<ForexItem>,
    pub EURMXN: Option<ForexItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForexItem {
    pub c: f64,
    pub m: f64,
    pub u: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndiceItem {
    pub a: f64,
    pub c: f64,
    pub e: String,
    pub f: String,
    pub m: f64,
    pub n: f64,
    pub u: f64,
    pub v: f64,
    pub x: f64,
    pub ytdp: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndicesResponse {
    pub SP500: Option<IndiceItem>,
    pub FTSEBIVA: Option<IndiceItem>,
    pub IPC: Option<IndiceItem>,
    pub DJIA: Option<IndiceItem>,
}

#[derive(Debug, Deserialize)]
pub struct TasaItem {
    pub f: String,
    pub t: f64,
}

#[derive(Debug, Deserialize)]
pub struct TasasResponse {
    pub CETE364: Option<TasaItem>,
    pub TIIE91: Option<TasaItem>,
    pub TIIE182: Option<TasaItem>,
    pub CETE182: Option<TasaItem>,
    pub CETE28: Option<TasaItem>,
    pub TIIEFB: Option<TasaItem>,
    pub TIIE28: Option<TasaItem>,
    #[serde(rename = "CETE 91")]
    pub CETE_91: Option<TasaItem>,
    pub Tasa_Objetivo: Option<TasaItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmisoraBusqueda {
    pub razon_social: String,
    pub emisoras: String, 
    pub serie: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleTopChange {
    pub e: String, 
    pub c: f64,    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleTopResponse {
    pub suben: Vec<SimpleTopChange>,
    pub bajan: Vec<SimpleTopChange>,
}


fn parse_json(s: Option<String>) -> Option<serde_json::Value> {
    s.and_then(|json_str| serde_json::from_str(&json_str).ok())
}

fn get_api_key() -> String {
    dotenv().ok();
    env::var("API_KEY").expect("API_KEY not set in .env")
}

pub fn get_ticker(pg_client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    let url = format!("https://api.databursatil.com/v2/emisoras?token={}", api_key);
    let client = HttpClient::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;

    let map: HashMap<String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("Total tickers recibidos: {}", map.len());
    let mut guardados = 0;
    let mut errores = 0;
    for (ticker, inner_obj) in map {
        println!("Procesando ticker: {}", ticker);
        if let Some(obj) = inner_obj.as_object() {
            for (serie, emisora_val) in obj {
                if emisora_val.is_array() {
                    if let Some(arr) = emisora_val.as_array() {
                        for sub_emisora_val in arr {
                            if let Some(emisora_obj) = sub_emisora_val.as_object() {
                                let razon_social = emisora_obj.get("razon_social").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let isin = emisora_obj.get("isin").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let bolsa = emisora_obj.get("bolsa").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let tipo_valor = emisora_obj.get("tipo_valor_descripcion").and_then(|v| v.as_str()).map(|s| s.to_string());
                                let tipo_valor_id = emisora_obj.get("tipo_valor_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let estatus = emisora_obj.get("estatus").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let acciones_circulacion = emisora_obj.get("acciones_en_circulacion").and_then(|v| v.as_i64());
                                let rangos_historicos = emisora_obj.get("rango_historicos").and_then(|v| v.as_str()).map(|s| s.to_string());
                                let rangos_financieros = emisora_obj.get("rango_financieros").and_then(|v| v.as_str()).map(|s| s.to_string());
                                let dividendos = emisora_obj.get("dividendos").cloned();
                                let rangos_historicos_str = rangos_historicos.clone();
                                let rangos_financieros_str = rangos_financieros.clone();
                                let dividendos_str = dividendos.as_ref().map(|v| serde_json::to_string(v).unwrap_or_default());
                                let rows = pg_client.query(
                                    "SELECT 1 FROM emisoras WHERE emisoras = $1 AND serie = $2",
                                    &[&ticker, &serie],
                                )?;
                                if rows.is_empty() {
                                    match pg_client.execute(
                                        "INSERT INTO emisoras (
                                            emisoras, serie, razon_social, isin, bolsa, tipo_valor, tipo_valor_id, estatus,
                                            acciones_circulacion, rangos_historicos, rangos_financieros, dividendos
                                        ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)",
                                        &[
                                            &ticker,
                                            &serie,
                                            &razon_social,
                                            &isin,
                                            &bolsa,
                                            &tipo_valor,
                                            &tipo_valor_id,
                                            &estatus,
                                            &acciones_circulacion,
                                            &rangos_historicos_str,
                                            &rangos_financieros_str,
                                            &dividendos_str,
                                        ],
                                    ) {
                                        Ok(_) => { println!("Insertado: {} ({})", ticker, serie); guardados += 1; },
                                        Err(e) => { println!("Error insertando {} ({}): {}\nDatos: {}", ticker, serie, e, serde_json::to_string(&emisora_obj).unwrap_or_default()); errores += 1; },
                                    }
                                } else {
                                    match pg_client.execute(
                                        "UPDATE emisoras SET
                                            razon_social = $3,
                                            isin = $4,
                                            bolsa = $5,
                                            tipo_valor = $6,
                                            tipo_valor_id = $7,
                                            estatus = $8,
                                            acciones_circulacion = $9,
                                            rangos_historicos = $10,
                                            rangos_financieros = $11,
                                            dividendos = $12
                                         WHERE emisoras = $1 AND serie = $2",
                                        &[
                                            &ticker,
                                            &serie,
                                            &razon_social,
                                            &isin,
                                            &bolsa,
                                            &tipo_valor,
                                            &tipo_valor_id,
                                            &estatus,
                                            &acciones_circulacion,
                                            &rangos_historicos_str,
                                            &rangos_financieros_str,
                                            &dividendos_str,
                                        ],
                                    ) {
                                        Ok(_) => { println!("Actualizado: {} ({})", ticker, serie); guardados += 1; },
                                        Err(e) => { println!("Error actualizando {} ({}): {}\nDatos: {}", ticker, serie, e, serde_json::to_string(&emisora_obj).unwrap_or_default()); errores += 1; },
                                    }
                                }
                            }
                        }
                    }
                } else if let Some(emisora_obj) = emisora_val.as_object() {
                    let razon_social = emisora_obj.get("razon_social").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let isin = emisora_obj.get("isin").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let bolsa = emisora_obj.get("bolsa").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let tipo_valor = emisora_obj.get("tipo_valor_descripcion").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let tipo_valor_id = emisora_obj.get("tipo_valor_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let estatus = emisora_obj.get("estatus").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let acciones_circulacion = emisora_obj.get("acciones_en_circulacion").and_then(|v| v.as_i64());
                    let rangos_historicos = emisora_obj.get("rango_historicos").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let rangos_financieros = emisora_obj.get("rango_financieros").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let dividendos = emisora_obj.get("dividendos").cloned();
                    let rangos_historicos_str = rangos_historicos.clone();
                    let rangos_financieros_str = rangos_financieros.clone();
                    let dividendos_str = dividendos.as_ref().map(|v| serde_json::to_string(v).unwrap_or_default());
                    let rows = pg_client.query(
                        "SELECT 1 FROM emisoras WHERE emisoras = $1 AND serie = $2",
                        &[&ticker, &serie],
                    )?;
                    if rows.is_empty() {
                        match pg_client.execute(
                            "INSERT INTO emisoras (
                                emisoras, serie, razon_social, isin, bolsa, tipo_valor, tipo_valor_id, estatus,
                                acciones_circulacion, rangos_historicos, rangos_financieros, dividendos
                            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)",
                            &[
                                &ticker,
                                &serie,
                                &razon_social,
                                &isin,
                                &bolsa,
                                &tipo_valor,
                                &tipo_valor_id,
                                &estatus,
                                &acciones_circulacion,
                                &rangos_historicos_str,
                                &rangos_financieros_str,
                                &dividendos_str,
                            ],
                        ) {
                            Ok(_) => { println!("Insertado: {} ({})", ticker, serie); guardados += 1; },
                            Err(e) => { println!("Error insertando {} ({}): {}\nDatos: {}", ticker, serie, e, serde_json::to_string(&emisora_obj).unwrap_or_default()); errores += 1; },
                        }
                    } else {
                        match pg_client.execute(
                            "UPDATE emisoras SET
                                razon_social = $3,
                                isin = $4,
                                bolsa = $5,
                                tipo_valor = $6,
                                tipo_valor_id = $7,
                                estatus = $8,
                                acciones_circulacion = $9,
                                rangos_historicos = $10,
                                rangos_financieros = $11,
                                dividendos = $12
                             WHERE emisoras = $1 AND serie = $2",
                            &[
                                &ticker,
                                &serie,
                                &razon_social,
                                &isin,
                                &bolsa,
                                &tipo_valor,
                                &tipo_valor_id,
                                &estatus,
                                &acciones_circulacion,
                                &rangos_historicos_str,
                                &rangos_financieros_str,
                                &dividendos_str,
                            ],
                        ) {
                            Ok(_) => { println!("Actualizado: {} ({})", ticker, serie); guardados += 1; },
                            Err(e) => { println!("Error actualizando {} ({}): {}\nDatos: {}", ticker, serie, e, serde_json::to_string(&emisora_obj).unwrap_or_default()); errores += 1; },
                        }
                    }
                }
            }
        } else {
            println!("No se encontró objeto interno para ticker: {}", ticker);
            errores += 1;
        }
    }
    println!("Guardados correctamente: {} | Errores: {}", guardados, errores);
    Ok(())
}

pub fn show_data(pg_client: &mut Client)-> Result<(), Box<dyn std::error::Error>> {
    for row in pg_client.query(
        "SELECT \
            emisoras, \
            serie, \
            razon_social, \
            isin, \
            bolsa, \
            tipo_valor, \
            tipo_valor_id, \
            estatus, \
            acciones_circulacion, \
            rangos_historicos, \
            rangos_financieros, \
            dividendos \
        FROM emisoras",
        &[]
    )? {
        let emisora = Emisora {
            razon_social: row.get(2),
            isin: row.get(3),
            bolsa: row.get(4),
            tipo_valor: row.get(5),
            tipo_valor_id: row.get(6),
            estatus: row.get(7),
            acciones_circulacion: row.get(8),
            rangos_historicos: row.get(9),
            rangos_financieros: row.get(10),
            dividendos: parse_json(row.get(11)),
        };
        println!("{:#?}", emisora);
    }

    Ok(())
}

fn construir_url_intradia(emisoras: &[&str], inicio: &str, final_: &str, api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    if emisoras.is_empty() {
        return Err("Lista de emisoras vacía".into());
    }
    let emisoras_str = emisoras.join(",");
    let url = format!(
        "https://api.databursatil.com/v2/intradia?token={}&emisora_serie={}&bolsa=BMV&intervalo=1h&inicio={}&final={}",
        api_key, emisoras_str, inicio, final_
    );
    Ok(url)
}

pub fn get_intradia(emi: &[&str], ini: &str, fin: &str, pg_client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    let url = construir_url_intradia(emi, ini, fin, &api_key)?;
    let client = HttpClient::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    
    let map: HashMap<String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("Intradía: {:#?}", map);
    
    for (ticker, inner_obj) in map {
        if let serde_json::Value::Object(map_inner) = inner_obj {
            for (fecha_hora_str, precio_val) in map_inner {
                if let serde_json::Value::Number(precio_num) = precio_val {
                    if let Some(precio) = precio_num.as_f64() {
                        let fecha_hora = NaiveDateTime::parse_from_str(
                            &fecha_hora_str, 
                            "%Y-%m-%d %H:%M:%S"
                        )?;

                        match pg_client.execute(
                            "INSERT INTO intradia_data (emisora, fecha_hora, precio)
                             VALUES ($1, $2, $3)
                             ON CONFLICT (emisora, fecha_hora)
                             DO UPDATE SET precio = EXCLUDED.precio",
                            &[&ticker, &fecha_hora, &precio],
                        ) {
                            Ok(_) => println!("Datos insertados/actualizados para {} a las {}", ticker, fecha_hora),
                            Err(e) => println!("Error en BD para {}: {}", ticker, e),
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_cotizaciones(emisora: &str) -> Result<Option<Cotizacion>, Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    let url = format!(
        "https://api.databursatil.com/v2/cotizaciones?token={}&emisora_serie={}&concepto=p,v,u&bolsa=bmv",
        api_key, emisora
    );
    let client = HttpClient::new();

    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;

    let map: HashMap<String, serde_json::Value> = serde_json::from_str(&response)?;

    for (ticker, inner_obj) in map {
        if let serde_json::Value::Object(bolsas) = inner_obj {
            for (_bolsa, valores) in bolsas {
                if let serde_json::Value::Object(inner_jsn) = valores {
                    let mut ultimo_precio = None;
                    let mut precio_promedio = None;
                    let mut volumen = None;
                    let mut fecha = None;

                    for (simbol, value) in &inner_jsn {
                        match (simbol.as_str(), value) {
                            ("u", serde_json::Value::Number(num)) => {
                                ultimo_precio = num.as_f64();
                            }
                            ("p", serde_json::Value::Number(num)) => {
                                precio_promedio = num.as_f64();
                            }
                            ("v", serde_json::Value::Number(num)) => {
                                volumen = num.as_f64();
                            }
                            ("f", serde_json::Value::String(s)) => {
                                fecha = Some(s.clone());
                            }
                            _ => {}
                        }
                    }

                    let cotizacion = Cotizacion {
                        simbolo: ticker.clone(),
                        ultimo_precio,
                        precio_promedio,
                        volumen,
                        fecha,
                    };
                    return Ok(Some(cotizacion));
                }
            }
        }
    }

    Ok(None)
}

pub fn get_top() -> Result<TopResponse, Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    let now = Local::now();
    let mut fecha = now.date_naive();
    if now.hour() < 7 {
        fecha = fecha.pred();
    }
    let fecha_str = fecha.format("%Y-%m-%d").to_string();
    let url = format!(
        "https://api.databursatil.com/v2/top?token={}&variables=suben,bajan,importe,volumen,operaciones&bolsa=BMV&cantidad=5&mercado=local&inicio={fecha}&final={fecha}",
        api_key, fecha = fecha_str
    );
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;

    let map: serde_json::Value = serde_json::from_str(&response)?;

    fn safe_vec<T: for<'a> serde::Deserialize<'a>>(v: Option<serde_json::Value>) -> Result<Vec<T>, serde_json::Error> {
        match v {
            Some(serde_json::Value::Array(arr)) => serde_json::from_value(serde_json::Value::Array(arr)),
            _ => Ok(vec![]),
        }
    }

    let importe: Vec<TopImporte> = safe_vec(map.get("IMPORTE").cloned())?;
    let bajan: Vec<TopCambio> = safe_vec(map.get("BAJAN").cloned())?;
    let operaciones: Vec<TopOperaciones> = safe_vec(map.get("OPERACIONES").cloned())?;
    let suben: Vec<TopCambio> = safe_vec(map.get("SUBEN").cloned())?;
    let volumen: Vec<TopVolumen> = safe_vec(map.get("VOLUMEN").cloned())?;

    Ok(TopResponse {
        importe,
        bajan,
        operaciones,
        suben,
        volumen,
    })
}

#[command]
pub fn get_top_tauri() -> Result<TopResponse, String> {
    get_top().map_err(|e| e.to_string())
}



pub fn get_flujos_financieros(pg_client: &mut Client, emisora: &str, trimestre: &str) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    let url = format!(
        "https://api.databursatil.com/v2/financieros?token={}&emisora={}&periodo={}&financieros=flujos",
        api_key, emisora, trimestre
    );
    let client = HttpClient::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    let map: HashMap<String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("{:#?}", map);
    if let Some(serde_json::Value::Object(valores)) = map.get("flujos") {
        if let Some((periodo, datos)) = valores.iter().max_by_key(|(k, _)| *k) {
            let get_num = |key: &str| datos.get(key).and_then(|v| v.get(1)).and_then(|v| v.as_f64());
            let fecha_sql = Some(trimestre.to_string());
            let trimestre_sql = trimestre;
            let flujo_operacion = get_num("cashflowsfromusedinoperatingactivities");
            let utilidad_neta = get_num("profitloss");
            let depreciacion = get_num("adjustmentsfordepreciationandamortisationexpense");
            let cambio_inventarios = get_num("adjustmentsfordecreaseincreaseininventories");
            let cambio_cxc = get_num("adjustmentsfordecreaseincreaseintradeaccountreceivable");
            let cambio_cxp = get_num("adjustmentsforincreasedecreaseintradeaccountpayable");
            let impuestos_pagados = get_num("incometaxespaidrefundclassifiedasoperatingactivities");
            let intereses_pagados = get_num("interestpaidclassifiedasoperatingactivities");
            let flujo_inversion = get_num("cashflowsfromusedininvestingactivities");
            let capex = get_num("purchaseofpropertyplantandequipmentclassifiedasinvestingactivities");
            let venta_activos = get_num("proceedsfromsalesofpropertyplantandequipmentclassifiedasinvestingactivities");
            let compra_intangibles = get_num("purchaseofintangibleassetsclassifiedasinvestingactivities");
            let flujo_financiamiento = get_num("cashflowsfromusedinfinancingactivities");
            let prestamos_obtenidos = get_num("proceedsfromborrowingsclassifiedasfinancingactivities");
            let pago_deuda = get_num("repaymentsofborrowingsclassifiedasfinancingactivities");
            let dividendos_pagados = get_num("dividendspaidclassifiedasfinancingactivities");
            let recompras = get_num("paymentstoacquireorredeementitysshares");
            let cambio_efectivo = get_num("increasedecreaseincashandcashequivalents");
            let efectivo_final = get_num("cashandcashequivalents_ending");
            let efecto_tc = get_num("effectofexchangeratechangesoncashandcashequivalents");
            let deterioros = get_num("adjustmentsforimpairmentlossreversalofimpairmentlossrecognisedinprofitorloss");
            let partidas_no_monetarias = get_num("otheradjustmentsfornoncashitems");
            let costos_financieros = get_num("adjustmentsforfinancecosts");
            let fecha = periodo.split('_').last().unwrap_or("");
            let fecha_sql = match chrono::NaiveDate::parse_from_str(fecha, "%Y-%m-%d") {
                Ok(date) => Some(date),
                Err(_) => { return Ok(()); }
            };
            let query = "INSERT INTO estado_flujos (
                emisora, trimestre, fecha, flujo_operacion, utilidad_neta, depreciacion, cambio_inventarios, cambio_cxc, cambio_cxp, impuestos_pagados, intereses_pagados,
                flujo_inversion, capex, venta_activos, compra_intangibles,
                flujo_financiamiento, prestamos_obtenidos, pago_deuda, dividendos_pagados, recompras,
                cambio_efectivo, efectivo_final, efecto_tc,
                deterioros, partidas_no_monetarias, costos_financieros
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11,
                $12, $13, $14, $15,
                $16, $17, $18, $19, $20,
                $21, $22, $23,
                $24, $25, $26
            ) ON CONFLICT (emisora, trimestre) DO UPDATE SET
                fecha=EXCLUDED.fecha, flujo_operacion=EXCLUDED.flujo_operacion, utilidad_neta=EXCLUDED.utilidad_neta, depreciacion=EXCLUDED.depreciacion, cambio_inventarios=EXCLUDED.cambio_inventarios, cambio_cxc=EXCLUDED.cambio_cxc, cambio_cxp=EXCLUDED.cambio_cxp, impuestos_pagados=EXCLUDED.impuestos_pagados, intereses_pagados=EXCLUDED.intereses_pagados,
                flujo_inversion=EXCLUDED.flujo_inversion, capex=EXCLUDED.capex, venta_activos=EXCLUDED.venta_activos, compra_intangibles=EXCLUDED.compra_intangibles,
                flujo_financiamiento=EXCLUDED.flujo_financiamiento, prestamos_obtenidos=EXCLUDED.prestamos_obtenidos, pago_deuda=EXCLUDED.pago_deuda, dividendos_pagados=EXCLUDED.dividendos_pagados, recompras=EXCLUDED.recompras,
                cambio_efectivo=EXCLUDED.cambio_efectivo, efectivo_final=EXCLUDED.efectivo_final, efecto_tc=EXCLUDED.efecto_tc,
                deterioros=EXCLUDED.deterioros, partidas_no_monetarias=EXCLUDED.partidas_no_monetarias, costos_financieros=EXCLUDED.costos_financieros";
            let params: [&(dyn postgres::types::ToSql + Sync); 26] = [
                &emisora,
                &trimestre_sql,
                &fecha_sql,
                &flujo_operacion,
                &utilidad_neta,
                &depreciacion,
                &cambio_inventarios,
                &cambio_cxc,
                &cambio_cxp,
                &impuestos_pagados,
                &intereses_pagados,
                &flujo_inversion,
                &capex,
                &venta_activos,
                &compra_intangibles,
                &flujo_financiamiento,
                &prestamos_obtenidos,
                &pago_deuda,
                &dividendos_pagados,
                &recompras,
                &cambio_efectivo,
                &efectivo_final,
                &efecto_tc,
                &deterioros,
                &partidas_no_monetarias,
                &costos_financieros
            ];
            let _ = pg_client.execute(query, &params);
        }
    }
    Ok(())
}


pub fn get_estado_resultado_trimestral(
    pg_client: &mut Client,
    emisora: &str,
    trimestre: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    let url = format!(
        "https://api.databursatil.com/v2/financieros?token={}&emisora={}&periodo={}&financieros=resultado_trimestre",
        api_key, emisora, trimestre
    );
    let client = HttpClient::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    
    let map: HashMap<String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("{:#?}",map);

    
    if let Some(serde_json::Value::Object(valores)) = map.get("resultado_trimestre") {
        if let Some((periodo, datos)) = valores.iter().max_by_key(|(k, _)| *k) {
            let get_num = |key: &str| -> Option<f64> {
                datos.get(key)
                    .and_then(|v| v.get(1))
                    .and_then(|v| v.as_f64())
            };
            
            let fecha = periodo.split('_').last()
                .and_then(|fecha_str| {
                    if fecha_str.is_empty() {
                        None
                    } else {                        
                            fecha_str.parse::<i32>().ok().and_then(|year| {
                            NaiveDate::from_ymd_opt(year, 1, 1)
                        })
                    }
                });
            let revenue = get_num("revenue");
            let grossprofit = get_num("grossprofit");
            let profitlossfromoperatingactivities = get_num("profitlossfromoperatingactivities");
            let profitloss = get_num("profitloss");
            let profitlossbeforetax = get_num("profitlossbeforetax");
            let costofsales = get_num("costofsales");
            let distributioncosts = get_num("distributioncost");
            let administrativeexpense = get_num("administrativeexpense");
            let financecosts = get_num("financecosts");
            let financeincome = get_num("financeincome");
            let incometaxexpensecontinuingoperations = get_num("incometaxexpensecontinuingoperations");
            let profitlossattributabletoownersofparent = get_num("profitlossattributabletoownersofparent");
            let basicearningslosspershare = get_num("basicearningslosspershare");
            let dilutedearningslosspershare = get_num("dilutedearningslosspershare");
            let otherincome = get_num("otherincome");
            let shareofprofitlossofassociatesandjointventuresaccountedforusinge = get_num("shareofprofitlossofassociatesandjointventuresaccountedforusinge");
            let profitlossfromdiscontinuedoperations = get_num("profitlossfromdiscontinuedoperations");
            let depreciacion = get_num("depreciacion");
            let fecha_sql = fecha;
            let query = r#"
                INSERT INTO estado_resultado_trimestral (
                    emisora, trimestre, fecha, revenue, grossprofit, 
                    profitlossfromoperatingactivities, profitloss, profitlossbeforetax, 
                    costofsales, distributioncosts, administrativeexpense, financecosts, 
                    financeincome, incometaxexpensecontinuingoperations, 
                    profitlossattributabletoownersofparent, basicearningslosspershare, 
                    dilutedearningslosspershare, otherincome, 
                    shareofprofitlossofassociatesandjointventuresaccountedforusinge,
                    profitlossfromdiscontinuedoperations, depreciacion
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, 
                    $16, $17, $18, $19, $20, $21
                ) 
                ON CONFLICT (emisora, trimestre) 
                DO UPDATE SET
                    fecha = EXCLUDED.fecha,
                    revenue = EXCLUDED.revenue,
                    grossprofit = EXCLUDED.grossprofit,
                    profitlossfromoperatingactivities = EXCLUDED.profitlossfromoperatingactivities,
                    profitloss = EXCLUDED.profitloss,
                    profitlossbeforetax = EXCLUDED.profitlossbeforetax,
                    costofsales = EXCLUDED.costofsales,
                    distributioncosts = EXCLUDED.distributioncosts,
                    administrativeexpense = EXCLUDED.administrativeexpense,
                    financecosts = EXCLUDED.financecosts,
                    financeincome = EXCLUDED.financeincome,
                    incometaxexpensecontinuingoperations = EXCLUDED.incometaxexpensecontinuingoperations,
                    profitlossattributabletoownersofparent = EXCLUDED.profitlossattributabletoownersofparent,
                    basicearningslosspershare = EXCLUDED.basicearningslosspershare,
                    dilutedearningslosspershare = EXCLUDED.dilutedearningslosspershare,
                    otherincome = EXCLUDED.otherincome,
                    shareofprofitlossofassociatesandjointventuresaccountedforusinge = EXCLUDED.shareofprofitlossofassociatesandjointventuresaccountedforusinge,
                    profitlossfromdiscontinuedoperations = EXCLUDED.profitlossfromdiscontinuedoperations,
                    depreciacion = EXCLUDED.depreciacion
            "#;

            pg_client.execute(
                query,
                &[
                    &emisora,
                    &trimestre,
                    &fecha_sql, 
                    &revenue,
                    &grossprofit,
                    &profitlossfromoperatingactivities,
                    &profitloss,
                    &profitlossbeforetax,
                    &costofsales,
                    &distributioncosts,
                    &administrativeexpense,
                    &financecosts,
                    &financeincome,
                    &incometaxexpensecontinuingoperations,
                    &profitlossattributabletoownersofparent,
                    &basicearningslosspershare,
                    &dilutedearningslosspershare,
                    &otherincome,
                    &shareofprofitlossofassociatesandjointventuresaccountedforusinge,
                    &profitlossfromdiscontinuedoperations,
                    &depreciacion,
                ],
            )?;
        }
    }
    Ok(())
}


pub fn get_posicion_financiera(
    pg_client: &mut Client,
    emisora: &str,
    trimestre: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    let url = format!(
        "https://api.databursatil.com/v2/financieros?token={}&emisora={}&periodo={}&financieros=posicion",
        api_key, emisora, trimestre
    );
    let client = HttpClient::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    
    let map: HashMap<String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("{:#?}",map);

    if let Some(serde_json::Value::Object(valores)) = map.get("posicion") {
        if let Some((periodo, datos)) = valores.iter().max_by_key(|(k, _)| *k) {
            let get_num = |key: &str| -> Option<f64> {
                datos.get(key)
                    .and_then(|v| v.get(1))
                    .and_then(|v| v.as_f64())
            };
            
            let fecha = periodo.split('_').last()
                .and_then(|fecha_str| {
                    if fecha_str.is_empty() {
                        None
                    } else {
                        NaiveDate::parse_from_str(fecha_str, "%Y-%m-%d").ok()
                    }
                });
            let currentassets = get_num("currentassets");
            let currentliabilities = get_num("currentliabilities");
            let cashandcashequivalents = get_num("cashandcashequivalents");
            let inventories = get_num("inventories");
            let tradeandothercurrentreceivables = get_num("tradeandothercurrentreceivables");
            let tradeandothercurrentpayables = get_num("tradeandothercurrentpayables");
            let equity = get_num("equity");
            let liabilities = get_num("liabilities");
            let noncurrentliabilities = get_num("noncurrentliabilities");
            let equityattributabletoownersofparent = get_num("equityattributabletoownersofparent");
            let noncontrollinginterests = get_num("noncontrollinginterests");
            let propertyplantandequipment = get_num("propertyplantandequipment");
            let intangibleassetsotherthangoodwill = get_num("intangibleassetsotherthangoodwill");
            let goodwill = get_num("goodwill");
            let rightofuseassetsthatdonotmeetdefinitionofinvestmentproperty = get_num("rightofuseassetsthatdonotmeetdefinitionofinvestmentproperty");
            let deferredtaxassets = get_num("deferredtaxassets");
            let deferredtaxliabilities = get_num("deferredtaxliabilities");
            let noncurrentassetsordisposalgroupsclassifiedasheldforsale = get_num("noncurrentassetsordisposalgroupsclassifiedasheldforsale");
            let retainedearnings = get_num("retainedearnings");
            let issuedcapital = get_num("issuedcapital");
            let otherreserves = get_num("otherreserves");
            let noncurrentleaseliabilities = get_num("noncurrentleaseliabilities");
            let othernoncurrentfinancialliabilities = get_num("othernoncurrentfinancialliabilities");
            let noncurrentprovisionsforemployeebenefits = get_num("noncurrentprovisionsforemployeebenefits");
            let query = r#"
                INSERT INTO estado_posicion (
                    emisora, trimestre, fecha, currentassets, currentliabilities, cashandcashequivalents, inventories, tradeandothercurrentreceivables, tradeandothercurrentpayables, equity, liabilities, noncurrentliabilities, equityattributabletoownersofparent, noncontrollinginterests, propertyplantandequipment, intangibleassetsotherthangoodwill, goodwill, rightofuseassetsthatdonotmeetdefinitionofinvestmentproperty, deferredtaxassets, deferredtaxliabilities, noncurrentassetsordisposalgroupsclassifiedasheldforsale, retainedearnings, issuedcapital, otherreserves, noncurrentleaseliabilities, othernoncurrentfinancialliabilities, noncurrentprovisionsforemployeebenefits
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27
                )
                ON CONFLICT (emisora, trimestre)
                DO UPDATE SET
                    fecha = EXCLUDED.fecha,
                    currentassets = EXCLUDED.currentassets,
                    currentliabilities = EXCLUDED.currentliabilities,
                    cashandcashequivalents = EXCLUDED.cashandcashequivalents,
                    inventories = EXCLUDED.inventories,
                    tradeandothercurrentreceivables = EXCLUDED.tradeandothercurrentreceivables,
                    tradeandothercurrentpayables = EXCLUDED.tradeandothercurrentpayables,
                    equity = EXCLUDED.equity,
                    liabilities = EXCLUDED.liabilities,
                    noncurrentliabilities = EXCLUDED.noncurrentliabilities,
                    equityattributabletoownersofparent = EXCLUDED.equityattributabletoownersofparent,
                    noncontrollinginterests = EXCLUDED.noncontrollinginterests,
                    propertyplantandequipment = EXCLUDED.propertyplantandequipment,
                    intangibleassetsotherthangoodwill = EXCLUDED.intangibleassetsotherthangoodwill,
                    goodwill = EXCLUDED.goodwill,
                    rightofuseassetsthatdonotmeetdefinitionofinvestmentproperty = EXCLUDED.rightofuseassetsthatdonotmeetdefinitionofinvestmentproperty,
                    deferredtaxassets = EXCLUDED.deferredtaxassets,
                    deferredtaxliabilities = EXCLUDED.deferredtaxliabilities,
                    noncurrentassetsordisposalgroupsclassifiedasheldforsale = EXCLUDED.noncurrentassetsordisposalgroupsclassifiedasheldforsale,
                    retainedearnings = EXCLUDED.retainedearnings,
                    issuedcapital = EXCLUDED.issuedcapital,
                    otherreserves = EXCLUDED.otherreserves,
                    noncurrentleaseliabilities = EXCLUDED.noncurrentleaseliabilities,
                    othernoncurrentfinancialliabilities = EXCLUDED.othernoncurrentfinancialliabilities,
                    noncurrentprovisionsforemployeebenefits = EXCLUDED.noncurrentprovisionsforemployeebenefits
            "#;

            pg_client.execute(
                query,
                &[&emisora, 
                &trimestre,
                 &fecha, 
                 &currentassets, 
                 &currentliabilities, 
                 &cashandcashequivalents, 
                 &inventories, &tradeandothercurrentreceivables, 
                 &tradeandothercurrentpayables, 
                 &equity, 
                 &liabilities, 
                 &noncurrentliabilities, 
                 &equityattributabletoownersofparent, 
                 &noncontrollinginterests, 
                 &propertyplantandequipment, 
                 &intangibleassetsotherthangoodwill, 
                 &goodwill, 
                 &rightofuseassetsthatdonotmeetdefinitionofinvestmentproperty, 
                 &deferredtaxassets, 
                 &deferredtaxliabilities, 
                 &noncurrentassetsordisposalgroupsclassifiedasheldforsale, 
                 &retainedearnings, 
                 &issuedcapital, 
                 &otherreserves, 
                 &noncurrentleaseliabilities, 
                 &othernoncurrentfinancialliabilities, 
                 &noncurrentprovisionsforemployeebenefits],
            )?;
        }
    }
    Ok(())
}


pub fn get_indices() -> Result<IndicesResponse, Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    let url = format!(
        "https://api.databursatil.com/v2/indices?token={}&ticker=IPC,FTSEBIVA,SP500,DJIA",
        api_key
    );
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    let indices: IndicesResponse = serde_json::from_str(&response)?;
    Ok(indices)
}

#[command]
pub fn get_indices_tauri() -> Result<IndicesResponse, String> {
    get_indices().map_err(|e| e.to_string())
}

pub fn get_tasas_struct() -> Result<TasasResponse, Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    let url = format!(
        "https://api.databursatil.com/v2/tasas?token={}",
        api_key
    );
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    let tasas: TasasResponse = serde_json::from_str(&response)?;
    Ok(tasas)
}



pub fn get_forex() -> Result<ForexResponse, Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    let url = format!(
        "https://api.databursatil.com/v2/divisas?token={}&ticker=USDMXN,EURMXN",
        api_key
    );
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    let forex: ForexResponse = serde_json::from_str(&response)?;
    Ok(forex)
}

#[command]
pub fn get_forex_tauri() -> Result<ForexResponse, String> {
    get_forex().map_err(|e| e.to_string())
}

#[command]
pub fn buscar_emisoras(query: String) -> Result<Vec<EmisoraBusqueda>, String> {
    use postgres::{Client, NoTls};
    let mut client = Client::connect(
        "host=localhost user=garden_admin password=password dbname=dalia_db",
        NoTls,
    ).map_err(|e| e.to_string())?;
    let sql = r#"
        SELECT razon_social, emisoras, serie
        FROM emisoras
        WHERE LOWER(razon_social) LIKE $1 OR LOWER(emisoras) LIKE $1
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