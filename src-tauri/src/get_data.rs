use postgres::Client;
use reqwest::blocking::Client as HttpClient;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

fn parse_json(s: Option<String>) -> Option<serde_json::Value> {
    s.and_then(|json_str| serde_json::from_str(&json_str).ok())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emisora {
    pub razon_social: String,
    pub isin: String,
    pub bolsa: String,
    #[serde(rename = "tipo_valor")]
    pub tipo_valor: Option<String>,
    pub tipo_valor_id: String,
    pub estatus: String,
    #[serde(rename = "acciones_circulacion")]
    pub acciones_circulacion: Option<i64>,
    #[serde(rename = "rangos_historicos")]
    pub rangos_historicos: Option<serde_json::Value>,
    #[serde(rename = "rangos_financieros")]
    pub rangos_financieros: Option<serde_json::Value>,
    pub dividendos: Option<serde_json::Value>,
}

pub fn get_ticker(pg_client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.databursatil.com/v2/emisoras?token=10f433119085379e0dc544c3cd94e8";
    let client = HttpClient::new();
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;

    let map: HashMap<String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("Total tickers recibidos: {}", map.len());

    for (ticker, inner_obj) in map {
        println!("Procesando ticker: {}", ticker);
        if let Some((_, emisora_val)) = inner_obj.as_object().and_then(|o| o.iter().next()) {
            match serde_json::from_value::<Emisora>(emisora_val.clone()) {
                Ok(emisora) => {
                    println!("Emisora deserializada: {:?}", emisora);
                    let serie = String::new();

                    // Convertir campos JSON a Option<String>
                    let rangos_historicos_str = emisora.rangos_historicos
                        .as_ref()
                        .map(serde_json::to_string)
                        .transpose()?;
                        
                    let rangos_financieros_str = emisora.rangos_financieros
                        .as_ref()
                        .map(serde_json::to_string)
                        .transpose()?;
                        
                    let dividendos_str = emisora.dividendos
                        .as_ref()
                        .map(serde_json::to_string)
                        .transpose()?;

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
                                &emisora.razon_social,
                                &emisora.isin,
                                &emisora.bolsa,
                                &emisora.tipo_valor,
                                &emisora.tipo_valor_id,
                                &emisora.estatus,
                                &emisora.acciones_circulacion,
                                &rangos_historicos_str,
                                &rangos_financieros_str,
                                &dividendos_str,
                            ],
                        ) {
                            Ok(_) => println!("Insertado: {}", ticker),
                            Err(e) => println!("Error insertando {}: {}", ticker, e),
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
                                &emisora.razon_social,
                                &emisora.isin,
                                &emisora.bolsa,
                                &emisora.tipo_valor,
                                &emisora.tipo_valor_id,
                                &emisora.estatus,
                                &emisora.acciones_circulacion,
                                &rangos_historicos_str,
                                &rangos_financieros_str,
                                &dividendos_str,
                            ],
                        ) {
                            Ok(_) => println!("Actualizado: {}", ticker),
                            Err(e) => println!("Error actualizando {}: {}", ticker, e),
                        }
                    }
                }
                Err(e) => {
                    println!("Error deserializando emisora para {}: {}", ticker, e);
                }
            }
        } else {
            println!("No se encontró objeto interno para ticker: {}", ticker);
        }
    }

    Ok(())
}

pub fn show_data(pg_client: &mut Client)-> Result<(), Box<dyn std::error::Error>> {
    for row in pg_client.query(
        "SELECT 
            emisoras, 
            serie, 
            razon_social, 
            isin, 
            bolsa, 
            tipo_valor, 
            tipo_valor_id, 
            estatus, 
            acciones_circulacion, 
            rangos_historicos, 
            rangos_financieros, 
            dividendos 
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
            rangos_historicos: parse_json(row.get(9)),
            rangos_financieros: parse_json(row.get(10)),
            dividendos: parse_json(row.get(11)),
        };
        println!("{:#?}", emisora);
    }

    Ok(())
}

fn construir_url_intradia(emisoras: &[&str], inicio: &str, final_: &str) -> Result<String, Box<dyn std::error::Error>> {
    if emisoras.is_empty() {
        return Err("Lista de emisoras vacía".into());
    }
    let emisoras_str = emisoras.join(",");
    let url = format!(
        "https://api.databursatil.com/v2/intradia?token=10f433119085379e0dc544c3cd94e8&emisora_serie={}&bolsa=BMV&intervalo=1h&inicio={}&final={}",
        emisoras_str, inicio, final_
    );
    Ok(url)


}


pub fn get_intradia(emi: &[&str], ini: &str, fin: &str, pg_client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    let url = construir_url_intradia(emi, ini, fin)?;
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

pub fn get_cotizaciones(emisora: &str)-> Result<(), Box<dyn std::error::Error>>{
    let url = format!(
        "https://api.databursatil.com/v2/cotizaciones?token=10f433119085379e0dc544c3cd94e8&emisora_serie={}&concepto=u,p,v,f&bolsa=bmv",
        emisora
    );

    let client = HttpClient::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    //u = ultimo precio, f = fecha de precios, p = precio promedio porderado, v=volumen ponderado
    let map: HashMap <String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("Cotizaciones: {:#?}", map);
    Ok(())
}


pub fn get_top() -> Result<(), Box<dyn std::error::Error>>{
    let url = format!(
        "https://api.databursatil.com/v2/top?token=10f433119085379e0dc544c3cd94e8&variables=suben,bajan,importe,volumen,operaciones&bolsa=BMV&cantidad=5&mercado=local&inicio=2025-06-17&final=2025-06-17"
    );
    let client = HttpClient::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;

    let map: HashMap <String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("TOP: {:#?}", map);

    Ok(())
}

pub fn get_finantials(emisora: &str)->Result<(), Box<dyn std::error::Error>>{
    let url= format!(
        "https://api.databursatil.com/v2/financieros?token=10f433119085379e0dc544c3cd94e8&emisora={}&periodo=1T_2025&financieros=flujos",
        emisora
    );
    let client = HttpClient::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    let map: HashMap <String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("FINANCIEROS: {:#?}", map);
    Ok(())
}

pub fn get_idx()->Result<(), Box<dyn std::error::Error>>{
    let url = format!(
        "https://api.databursatil.com/v2/indices?token=10f433119085379e0dc544c3cd94e8&ticker=IPC,FTSEBIVA,SP500,DJIA"
    );
    let client = HttpClient::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    let map: HashMap <String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("Indices: {:#?}", map);
    Ok(())
}   


pub fn get_tasas()->Result<(), Box<dyn std::error::Error>>{
    let url = format!(
        "https://api.databursatil.com/v2/tasas?token=10f433119085379e0dc544c3cd94e8"
    );
    let client = HttpClient::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    let map: HashMap <String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("Tasas: {:#?}", map);
    Ok(())
}


pub fn get_forex()->Result<(), Box<dyn std::error::Error>>{
    let url = format!(
        "https://api.databursatil.com/v2/divisas?token=10f433119085379e0dc544c3cd94e8&ticker=USDMXN,EURMXN"
    );
    let client = HttpClient::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    let map: HashMap <String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("Divisas: {:#?}", map);
    Ok(())
}

pub fn get_comodities()->Result<(), Box<dyn std::error::Error>>{
    let url = format!(
        "https://api.databursatil.com/v2/commodities?token=10f433119085379e0dc544c3cd94e8"
    );
    let client = HttpClient::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    let map: HashMap <String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("Divisas: {:#?}", map);
    Ok(())

}


pub fn get_noticias()->Result<(), Box<dyn std::error::Error>>{
    let url = format!(
        "https://api.databursatil.com/v2/noticias?token=10f433119085379e0dc544c3cd94e8&"
    );
    let client = HttpClient::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    let map: HashMap <String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("Noticias: {:#?}", map);
    Ok(())
}


pub fn get_cables()->Result<(), Box<dyn std::error::Error>>{
    let url = format!(
        "https://api.databursatil.com/v2/cables?token=10f433119085379e0dc544c3cd94e8&categorias=valuaciones,top"
    );
    let client = HttpClient::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64)")
        .send()?
        .text()?;
    let map: HashMap <String, serde_json::Value> = serde_json::from_str(&response)?;
    println!("Noticias: {:#?}", map);
    Ok(())
}