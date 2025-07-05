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

fn ensure_user_exists(client: &mut Client, usuario_id: i32, nombre: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Intentar insertar el usuario
    let rows = client.execute(
        "INSERT INTO usuarios (id, nombre) VALUES ($1, $2) ON CONFLICT (id) DO NOTHING",
        &[&usuario_id, &nombre],
    )?;
    if rows == 0 {
        // Ya existía
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
    let host = env::var("DB_HOST").expect("DB_HOST not set");
    let user = env::var("DB_USER").expect("DB_USER not set");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD not set");
    let dbname = env::var("DB_NAME").expect("DB_NAME not set");
    let conn_str = format!(
        "host={} user={} password={} dbname={}",
        host, user, password, dbname
    );
    let mut client = Client::connect(&conn_str, NoTls).expect("No se pudo conectar a la base de datos");

    let usuario_id = 1;
    let nombre = "Makima";
    ensure_user_exists(&mut client, usuario_id, nombre).expect("No se pudo insertar usuario");
    //clean_emisoras::eliminar_duplicados_isin(&mut client).expect("No se pudo limpiar duplicados de ISIN");
    clean_emisoras::eliminar_duplicados_isin_hashset(&mut client).expect("No se pudo limpiar duplicados exactos de ISIN (HashSet)");

    // Obtener el primer portafolio del usuario automáticamente
    let portafolio_id = match portfolio::list_portfolios(&mut client, usuario_id) {
        Ok(portafolios) if !portafolios.is_empty() => portafolios[0].0, // id entero
        _ => {
            eprintln!("No hay portafolios para el usuario");
            return;
        }
    };
    // --- BORRAR TODOS LOS TICKERS Y TRANSACCIONES DEL PORTAFOLIO ---
    borrar_todos_los_tickers_y_transacciones(&mut client, portafolio_id);
    // --- FIN BORRADO ---
    let tickers = portfolio::list_tickers(&mut client, portafolio_id).unwrap_or_default();
    println!("\nAcciones actuales en el portafolio (id: {}):", portafolio_id);
    for (id, ticker, emisoras, serie) in &tickers {
        println!("id: {}, ticker: {}, emisoras: {}, serie: {}", id, ticker, emisoras, serie);
    }

    // --- AGREGAR GMEXICOB DE EJEMPLO ---
    //holdings::ejemplo_agregar_gmexicob(&mut client, portafolio_id);
    // --- FIN AGREGADO ---

    // --- LISTAR HOLDINGS ---
    holdings::listar_holdings(&mut client, portafolio_id);
    // --- FIN LISTAR HOLDINGS ---

    // --- AGREGAR USUARIO Y PORTAFOLIO ---
    let _id_hex = holdings::agregar_usuario_y_portafolio(&mut client, 1, "makima", "dalia").expect("No se pudo crear usuario y portafolio");

    let resultado = get_data::buscar_emisoras("LIVEPOL".to_string());
    match resultado {
        Ok(lista) => {
            println!("\nResultados de buscar_emisoras para 'KOF':");
            for emisora in lista {
                println!("{} | {} | {}", emisora.razon_social, emisora.emisoras, emisora.serie);
            }
        },
        Err(e) => println!("Error en buscar_emisoras: {}", e),
    }
    // --- FIN EJEMPLO ---

    // --- EJEMPLO DE USO DE get_finantial_flow, get_finantial_position y get_quarterly_income_statement ---
    let emisora = "AMX";
    let trimestre = "1T_2025";

    match activos::get_finantial_flow(&mut client, emisora, trimestre) {
        Ok(flujo) => {
            println!("\nFlujo financiero de {} en {}:", emisora, trimestre);
            for (k, v) in &flujo {
                println!("{}: {}", k, v);
            }
        },
        Err(e) => println!("Error al obtener flujo financiero: {}", e),
    }

    match activos::get_finantial_position(&mut client, emisora, trimestre) {
        Ok(posicion) => {
            println!("\nPosición financiera de {} en {}:", emisora, trimestre);
            for (k, v) in &posicion {
                println!("{}: {}", k, v);
            }
        },
        Err(e) => println!("Error al obtener posición financiera: {}", e),
    }

    match activos::get_quarterly_income_statement(&mut client, emisora, trimestre) {
        Ok(estado) => {
            println!("\nEstado de resultados trimestral de {} en {}:", emisora, trimestre);
            for (k, v) in &estado {
                println!("{}: {}", k, v);
            }
        },
        Err(e) => println!("Error al obtener estado de resultados trimestral: {}", e),
    }
    // --- FIN EJEMPLO ---
}