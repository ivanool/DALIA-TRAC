#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use postgres::{Client, NoTls};
use std::env;
mod get_data;
mod portfolio;
use get_data::{get_tasas_struct, get_forex, get_indices};
use tauri::Builder;

fn ensure_user_exists(client: &mut Client, usuario_id: i32, nombre: &str) -> Result<(), Box<dyn std::error::Error>> {
    client.execute(
        "INSERT INTO usuarios (id, nombre) VALUES ($1, $2) ON CONFLICT (id) DO NOTHING",
        &[&usuario_id, &nombre],
    )?;
    Ok(())
}

fn main() {
    dotenv().ok();
    let host = env::var("DB_HOST").expect("DB_HOST not set");
    let user = env::var("DB_USER").expect("DB_USER not set");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD not set");
    let dbname = env::var("DB_NAME").expect("DB_NAME not set");
    let conn_str = format!(
        "host={} user={} password={} dbname={}",
        host, user, password, dbname
    );
    let mut client = Client::connect(&conn_str, NoTls).expect("No se pudo conectar a la base de datos");

    // Asegurar que el usuario existe
    let usuario_id = 1;
    let nombre = "Makima";
    ensure_user_exists(&mut client, usuario_id, nombre).expect("No se pudo insertar usuario");

    // Crear un portafolio
    let id_hex = crate::portfolio::add_portfolio(&mut client, usuario_id, nombre).expect("No se pudo crear portafolio");
    println!("Portafolio creado con id_hex: {}", id_hex);

    // Listar portafolios
    let portafolios = crate::portfolio::list_portfolios(&mut client, usuario_id).expect("No se pudieron listar portafolios");
    println!("Portafolios disponibles:");
    for (id, id_hex, nombre) in &portafolios {
        println!("id: {}, id_hex: {}, nombre: {}", id, id_hex, nombre);
    }

    // Añadir un activo al primer portafolio
    if let Some((id_portafolio, _, _)) = portafolios.first() {
        let ticker = "GMEXICO";
        let emisoras = "GMEXICO";
        let serie = "B";
        let id_ticker = crate::portfolio::add_ticker(&mut client, *id_portafolio, ticker, emisoras, serie).expect("No se pudo agregar ticker");
        println!("Ticker agregado con id: {}", id_ticker);

        // Listar tickers del portafolio
        let tickers = crate::portfolio::list_tickers(&mut client, *id_portafolio).expect("No se pudieron listar tickers");
        println!("Tickers en el portafolio:");
        for (id, ticker, emisoras, serie) in &tickers {
            println!("id: {}, ticker: {}, emisoras: {}, serie: {}", id, ticker, emisoras, serie);
        }

        // Agregar transacción al ticker
        crate::portfolio::add_transaction(&mut client, id_ticker, &"COMPRA".to_string(), 10, 200.0).expect("No se pudo agregar transacción");

        // Listar transacciones del ticker
        let transacciones = crate::portfolio::list_transactions(&mut client, id_ticker).expect("No se pudieron listar transacciones");
        println!("Transacciones:");
        for (id, tipo, cantidad, precio, fecha) in &transacciones {
            println!("id: {}, tipo: {}, cantidad: {}, precio: {}, fecha: {}", id, tipo, cantidad, precio, fecha);
        }

        // Agregar dividendo al ticker
        crate::portfolio::add_dividendo(&mut client, id_ticker, 50.0, None).expect("No se pudo agregar dividendo");

        // Listar dividendos por activo
        let dividendos_activo = crate::portfolio::list_dividendos_by_activo(&mut client, id_ticker).expect("No se pudieron listar dividendos por activo");
        println!("Dividendos por activo:");
        for (id, monto, fecha) in &dividendos_activo {
            println!("id: {}, monto: {}, fecha: {}", id, monto, fecha);
        }

        // Listar dividendos por portafolio
        let dividendos_portafolio = crate::portfolio::list_dividendos_by_portafolio(&mut client, *id_portafolio).expect("No se pudieron listar dividendos por portafolio");
        println!("Dividendos por portafolio:");
        for (id, portafolio_ticker_id, monto, fecha) in &dividendos_portafolio {
            println!("id: {}, portafolio_ticker_id: {}, monto: {}, fecha: {}", id, portafolio_ticker_id, monto, fecha);
        }

        // Agregar movimiento de cashflow
        crate::portfolio::add_cashflow(&mut client, *id_portafolio, 1000.0, "DEPOSITO", Some("Depósito inicial"), None).expect("No se pudo agregar cashflow");

        // Listar movimientos de cashflow
        let cashflows = crate::portfolio::list_cashflow(&mut client, *id_portafolio).expect("No se pudieron listar cashflows");
        println!("Cashflows:");
        for (id, monto, tipo, fecha, descripcion) in &cashflows {
            println!("id: {}, monto: {}, tipo: {}, fecha: {}, descripcion: {:?}", id, monto, tipo, fecha, descripcion);
        }

        // Obtener saldo de cashflow
        let saldo = crate::portfolio::get_cashflow_balance(&mut client, *id_portafolio).expect("No se pudo obtener saldo");
        println!("Saldo actual de cashflow: {}", saldo);
        
        // Filtrar cashflow por tipo
        let depositos = crate::portfolio::filter_cashflow(&mut client, *id_portafolio, Some("DEPOSITO"), None, None).expect("No se pudo filtrar cashflow");
        println!("Solo depósitos:");
        for (id, monto, tipo, fecha, descripcion) in &depositos {
            println!("id: {}, monto: {}, tipo: {}, fecha: {}, descripcion: {:?}", id, monto, tipo, fecha, descripcion);
        }
    }
}







fn main_test() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::connect(
        "host=localhost user=garden_admin password=password dbname=dalia_db",
        NoTls,
    )?;

    let _emisoras = ["KOFUBL", "FEMSAUBD", "MEDICAB", "AMXB"];
    let _inicio = "2025-06-17";
    let _fin = "2025-06-17";

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