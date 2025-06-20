#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use postgres::{Client, NoTls};
mod get_data;
use get_data::{get_intradia, get_tasas_struct, get_forex, get_indices, get_top_tauri, buscar_emisoras};
use tauri::Builder;

fn main() {
    Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_top_tauri,
            buscar_emisoras,
            get_data::get_indices_tauri,
            get_data::get_forex_tauri
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::connect(
        "host=localhost user=garden_admin password=password dbname=dalia_db",
        NoTls,
    )?;

    let emisoras = ["KOFUBL", "FEMSAUBD", "MEDICAB", "AMXB"];
    let inicio = "2025-06-17";
    let fin = "2025-06-17";

    let ticker = "KOFUBL";
    match get_data::get_cotizaciones(ticker) {
        Ok(Some(cot)) => println!("{:?}", cot),
        Ok(None) => println!("No se encontró cotización para {}", ticker),
        Err(e) => println!("Error: {}", e),
    }

    match get_data::get_top() {
        Ok(top) => println!("{:#?}", top),
        Err(e) => println!("Error al obtener top: {}", e),
    }

    let tasas = get_tasas_struct()?;
    if let Some(tasa) = &tasas.Tasa_Objetivo {
        println!("Tasa objetivo: {} (fecha: {})", tasa.t, tasa.f);
    }

    let indices = get_indices()?;
    if let Some(ipc) = &indices.IPC {
        println!("IPC último: {}, cambio: {}", ipc.u, ipc.c);
    }

    let forex = get_forex()?;
    if let Some(usd) = &forex.USDMXN {
        println!("USDMXN: Último: {}, Cambio: {}, Monto: {}", usd.u, usd.c, usd.m);
    }

    Ok(())
}