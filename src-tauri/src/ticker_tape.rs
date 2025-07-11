use serde::Serialize;
use crate::get_data;

#[derive(Serialize)]
pub struct TickerData {
    pub symbol: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
}

#[tauri::command]
pub fn get_ticker_data() -> Result<Vec<TickerData>, String> {
    // --- Indices ---
    let indices = match get_data::get_indices() {
        Ok(i) => i,
        Err(e) => return Err(format!("Error obteniendo índices: {}", e)),
    };
    // --- Forex ---
    let forex = match get_data::get_forex() {
        Ok(f) => f,
        Err(e) => return Err(format!("Error obteniendo forex: {}", e)),
    };
    // --- Top movers ---
    let top = match get_data::get_top() {
        Ok(t) => t,
        Err(e) => return Err(format!("Error obteniendo top movers: {}", e)),
    };
    let mut data = Vec::new();
    // Indices principales
    if let Some(ipc) = indices.IPC {
        data.push(TickerData {
            symbol: "IPC".to_string(),
            price: ipc.u,
            change: 0.0,
            change_percent: ipc.c, // c es el porcentaje ya calculado
        });
    }
    if let Some(sp500) = indices.SP500 {
        data.push(TickerData {
            symbol: "S&P 500".to_string(),
            price: sp500.u,
            change: 0.0,
            change_percent: sp500.c, // c es el porcentaje ya calculado
        });
    }
    // Forex
    if let Some(usd) = forex.USDMXN {
        data.push(TickerData {
            symbol: "USD/MXN".to_string(),
            price: usd.u,
            change: 0.0,
            change_percent: usd.c, // c es el porcentaje ya calculado
        });
    }
    if let Some(eur) = forex.EURMXN {
        data.push(TickerData {
            symbol: "EUR/MXN".to_string(),
            price: eur.u,
            change: 0.0,
            change_percent: eur.c, // c es el porcentaje ya calculado
        });
    }
    // Top suben
    for sube in top.suben.iter().take(5) {
        data.push(TickerData {
            symbol: sube.e.clone(),
            price: sube.u,
            change: 0.0,
            change_percent: sube.c, // c es el porcentaje ya calculado
        });
    }
    // Top bajan
    for baja in top.bajan.iter().take(5) {
        data.push(TickerData {
            symbol: baja.e.clone(),
            price: baja.u,
            change: 0.0,
            change_percent: baja.c, // c es el porcentaje ya calculado
        });
    }
    Ok(data)
}
