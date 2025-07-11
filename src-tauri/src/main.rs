#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use postgres::{Client, NoTls};
use std::env;
mod get_data;
mod portfolio;
mod holdings; 
use tauri::Builder;
mod clean_emisoras;
mod activos; 
mod ticker_tape;

fn ensure_user_exists(client: &mut Client, usuario_id: i32, nombre: &str) -> Result<(), Box<dyn std::error::Error>> {
    let rows = client.execute(
        "INSERT INTO usuarios (id, nombre) VALUES ($1, $2) ON CONFLICT (id) DO NOTHING",
        &[&usuario_id, &nombre],
    )?;
    if rows == 0 {
        println!("Usuario '{}' (id: {}) ya existe. Iniciando sesión...", nombre, usuario_id);
    } else {
        println!("Usuario '{}' (id: {}) creado exitosamente.", nombre, usuario_id);
    }
    Ok(())
}

fn borrar_todos_los_tickers_y_transacciones(client: &mut Client, portafolio_id: i32) {
    let tickers = match portfolio::list_tickers(client, portafolio_id) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("[ERROR] No se pudieron listar los tickers: {}", e);
            return;
        }
    };
    for (id, ticker, emisoras, serie) in tickers {
        match portfolio::remove_ticker(client, id) {
            Ok(_) => println!("Ticker '{}' ({}/{}) eliminado.", ticker, emisoras, serie),
            Err(e) => eprintln!("[ERROR] No se pudo eliminar ticker '{}': {}", ticker, e),
        }
    }
    println!("Todos los tickers y transacciones del portafolio {} han sido eliminados.", portafolio_id);
}

fn main() {
    dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_data::get_indices_tauri,
            get_data::get_forex_tauri,
            get_data::get_top_tauri,
            get_data::buscar_emisoras,
            activos::get_emisora_query,
            activos::get_emisora_info,
            activos::get_trimestres_disponibles,
            activos::get_asset_details,
            ticker_tape::get_ticker_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}