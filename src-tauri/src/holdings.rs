use chrono::{Utc, TimeZone};
use postgres::Client;
use crate::portfolio;

/*
1.- Buscar los activos en la tabla de tickerportfolios, regresarlos



*/

pub fn ejemplo_agregar_gmexicob(client: &mut Client, portafolio_id: i32) {
    let ticker = "ANAUN";
    let emisoras = "ANAU";
    let serie = "N";
    // Usar DateTime<Utc> para timestamptz
    let fecha_alta_utc = Utc.with_ymd_and_hms(2025, 6, 24, 12, 0, 0).unwrap();
    let portafolio_ticker_id = match portfolio::add_ticker(client, portafolio_id, ticker, emisoras, serie, Some(fecha_alta_utc)) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("[ERROR SQL] No se pudo insertar el ticker '{}': {}", ticker, e);
            let tickers = portfolio::list_tickers(client, portafolio_id).unwrap_or_default();
            match tickers.iter().find(|(_, t, _, _)| t == ticker).map(|(id, _, _, _)| *id) {
                Some(id) => id,
                None => {
                    eprintln!("[ADVERTENCIA] No se encontró el ticker '{}' en el portafolio. No se agregará transacción de ejemplo.", ticker);
                    return;
                }
            }
        }
    };
    if let Err(e) = portfolio::add_transaction(client, portafolio_ticker_id, "compra", 1, 122.97) {
        eprintln!("[ADVERTENCIA] No se pudo agregar compra 1: {}", e);
    }
}

pub fn listar_holdings(client: &mut Client, portafolio_id: i32) {
    let tickers = portfolio::list_tickers(client, portafolio_id).unwrap_or_default();
    println!("\nSlots (holds) actuales en el portafolio (id: {}):", portafolio_id);
    for (id, ticker, emisoras, serie) in &tickers {
        // Consultar transacciones para este portafolio_ticker_id
        let trans = portfolio::list_transactions(client, *id).unwrap_or_default();
        let mut cantidad_total = 0;
        let mut monto_total = 0.0;
        let mut ventas_total = 0;
        let mut monto_ventas = 0.0;
        for (_tid, tipo, cantidad, precio, _fecha) in &trans {
            if tipo == "compra" {
                cantidad_total += cantidad;
                monto_total += (*cantidad as f64) * precio;
            } else if tipo == "venta" {
                ventas_total += cantidad;
                monto_ventas += (*cantidad as f64) * precio;
            }
        }
        let cantidad_actual = cantidad_total - ventas_total;
        let precio_promedio = if cantidad_total > 0 {
            monto_total / (cantidad_total as f64)
        } else {
            0.0
        };
        let precio_promedio_venta = if ventas_total > 0 {
            monto_ventas / (ventas_total as f64)
        } else {
            0.0
        };
        println!("id: {}, ticker: {}, emisoras: {}, serie: {} | HOLD: {} acciones | Precio compra prom: {:.2} | Vendidas: {} | Precio venta prom: {:.2}",
            id, ticker, emisoras, serie, cantidad_actual, precio_promedio, ventas_total, precio_promedio_venta);
    }
}

pub fn agregar_usuario_y_portafolio(client: &mut Client, usuario_id: i32, nombre_usuario: &str, nombre_portafolio: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 1. Agregar usuario (si no existe)
    client.execute(
        "INSERT INTO usuarios (id, nombre) VALUES ($1, $2) ON CONFLICT (id) DO NOTHING",
        &[&usuario_id, &nombre_usuario],
    )?;
    // 2. Verificar si el portafolio ya existe para ese usuario
    let row = client.query_opt(
        "SELECT id_hex FROM portafolios WHERE usuario_id = $1 AND nombre = $2",
        &[&usuario_id, &nombre_portafolio],
    )?;
    if let Some(row) = row {
        let id_hex: String = row.get(0);
        println!("Usuario '{}' (id: {}) y portafolio '{}' ya existen. id_hex: {}", nombre_usuario, usuario_id, nombre_portafolio, id_hex);
        return Ok(id_hex);
    }
    // 3. Crear portafolio para ese usuario si no existe
    let id_hex = portfolio::add_portfolio(client, usuario_id, nombre_portafolio)?;
    println!("Usuario '{}' (id: {}) y portafolio '{}' creados/vinculados.", nombre_usuario, usuario_id, nombre_portafolio);
    Ok(id_hex)
}
