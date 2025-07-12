use serde::{Deserialize, Serialize};
use postgres::{Client, NoTls, Error};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::get_data;

// --- DDL de referencia para la tabla intradia_data ---
/*
CREATE TABLE IF NOT EXISTS public.intradia_data (
    id integer NOT NULL DEFAULT nextval('intradia_data_id_seq'::regclass),
    fecha_hora timestamp without time zone,
    precio double precision,
    emisora character varying,
    emisoras text,
    serie text
);
*/

#[derive(Serialize)]
pub struct Holding {
    ticker: String,
    quantity: f64,
    average_cost: f64,
    market_value: f64,
    unrealized_pnl: f64,
    unrealized_pnl_percent: f64,
}

#[derive(Serialize)]
pub struct PortfolioSummary {
    total_value: f64,
    total_pnl: f64,
    total_pnl_percent: f64,
    holdings: Vec<Holding>,
}

// #[derive(Serialize)]
// pub struct TickerData {
//     pub symbol: String,
//     pub price: f64,
//     pub change: f64,
//     pub change_percent: f64,
// }

#[tauri::command]
pub fn add_portfolio_transaction(
    user_id: i32,
    ticker: String,
    transaction_type: String,
    quantity: f64,
    price: f64,
    transaction_date: DateTime<Utc>
) -> Result<(), String> {
    let mut client = connect_db()?;
    let mut db_transaction = client.transaction().map_err(|e| e.to_string())?;

    let total_amount = quantity * price;

    db_transaction.execute(
        "INSERT INTO portfolio_transactions (user_id, ticker, transaction_type, quantity, price, transaction_date, total_amount) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[&user_id, &ticker, &transaction_type, &quantity, &price, &transaction_date, &total_amount],
    ).map_err(|e| e.to_string())?;

    db_transaction.commit().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_portfolio_summary(user_id: i32) -> Result<PortfolioSummary, String> {
    let mut client = connect_db()?;

    let mut holdings_map: HashMap<String, (f64, f64)> = HashMap::new();
    for row in client.query("SELECT ticker, quantity, total_amount FROM portfolio_transactions WHERE user_id = $1 AND transaction_type IN ('BUY', 'SELL')", &[&user_id]).map_err(|e| e.to_string())? {
        let ticker: String = row.get("ticker");
        let quantity: f64 = row.get("quantity");
        let total_amount: f64 = row.get("total_amount");

        let entry = holdings_map.entry(ticker).or_insert((0.0, 0.0));
        entry.0 += quantity;
        entry.1 += total_amount;
    }

    let mut holdings: Vec<Holding> = Vec::new();
    let mut total_portfolio_value = 0.0;
    let mut total_portfolio_cost = 0.0;

    for (ticker, (quantity, total_cost)) in holdings_map.iter() {
        if *quantity > 0.0 {
            let market_price = get_market_price(ticker).unwrap_or(0.0);
            let market_value = quantity * market_price;
            let average_cost = total_cost / quantity;
            let unrealized_pnl = market_value - total_cost;
            let unrealized_pnl_percent = if *total_cost > 0.0 { (unrealized_pnl / total_cost) * 100.0 } else { 0.0 };

            holdings.push(Holding {
                ticker: ticker.clone(),
                quantity: *quantity,
                average_cost,
                market_value,
                unrealized_pnl,
                unrealized_pnl_percent,
            });

            total_portfolio_value += market_value;
            total_portfolio_cost += total_cost;
        }
    }

    let total_pnl = total_portfolio_value - total_portfolio_cost;
    let total_pnl_percent = if total_portfolio_cost > 0.0 { (total_pnl / total_portfolio_cost) * 100.0 } else { 0.0 };

    Ok(PortfolioSummary {
        total_value: total_portfolio_value,
        total_pnl,
        total_pnl_percent,
        holdings,
    })
}

// --- Ticker Tape ---
// Mover a ticker_tape.rs

fn connect_db() -> Result<Client, String> {
    Client::connect("host=localhost user=garden_admin password=password dbname=dalia_db", NoTls)
        .map_err(|e| format!("DB connection error: {}", e))
}

fn get_market_price(ticker: &str) -> Result<f64, Error> {
    println!("Fetching market price for {}", ticker);
    Ok(150.75)
}

// --- FUNCIONES LEGACY RESTAURADAS PARA COMPATIBILIDAD ---

pub fn add_ticker(
    pg_client: &mut Client,
    portafolio_id: i32,
    ticker: &str,
    emisoras: &str,
    serie: &str,
    fecha: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<i32, Error> {
    if let Some(fecha) = fecha {
        let row = pg_client.query_one(
            "INSERT INTO portafolio_ticker (portafolio_id, ticker, emisoras, serie, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            &[&portafolio_id, &ticker, &emisoras, &serie, &fecha],
        )?;
        let id: i32 = row.get(0);
        Ok(id)
    } else {
        let row = pg_client.query_one(
            "INSERT INTO portafolio_ticker (portafolio_id, ticker, emisoras, serie) VALUES ($1, $2, $3, $4) RETURNING id",
            &[&portafolio_id, &ticker, &emisoras, &serie],
        )?;
        let id: i32 = row.get(0);
        Ok(id)
    }
}

pub fn list_tickers(
    pg_client: &mut Client,
    portafolio_id: i32
) -> Result<Vec<(i32, String, String, String)>, Error> {
    let rows = pg_client.query(
        "SELECT id, ticker, emisoras, serie FROM portafolio_ticker WHERE portafolio_id = $1 ORDER BY id",
        &[&portafolio_id],
    )?;
    let mut tickers = Vec::new();
    for row in rows {
        let id: i32 = row.get(0);
        let ticker: String = row.get(1);
        let emisoras: String = row.get(2);
        let serie: String = row.get(3);
        tickers.push((id, ticker, emisoras, serie));
    }
    Ok(tickers)
}

pub fn add_transaction(
    pg_client: &mut Client,
    portafolio_ticker_id: i32,
    tipo: &str,
    cantidad: i32,
    precio: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    pg_client.execute(
        "INSERT INTO transacciones (portafolio_ticker_id, tipo, cantidad, precio) VALUES ($1, $2, $3, $4)",
        &[&portafolio_ticker_id, &tipo, &cantidad, &precio],
    )?;
    Ok(())
}

pub fn list_transactions(
    pg_client: &mut Client,
    portfolio_ticker_id: i32
) -> Result<Vec<(i32, String, i32, f64, chrono::NaiveDateTime)>, Box<dyn std::error::Error>> {
    let rows = pg_client.query(
        "SELECT id, tipo, cantidad, precio, fecha FROM transacciones WHERE portafolio_ticker_id = $1 ORDER BY fecha DESC",
        &[&portfolio_ticker_id],
    )?;
    let mut transactions = Vec::new();
    for row in rows {
        let id: i32 = row.get(0);
        let tipo: String = row.get(1);
        let cantidad: i32 = row.get(2);
        let precio: f64 = row.get(3);
        let fecha: chrono::NaiveDateTime = row.get(4);
        transactions.push((id, tipo, cantidad, precio, fecha));
    }
    Ok(transactions)
}

pub fn remove_ticker(pg_client: &mut Client, id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let _ = pg_client.execute(
        "DELETE FROM portafolio_ticker WHERE id = $1",
        &[&id],
    )?;
    Ok(())
}

pub fn add_portfolio(
    pg_client: &mut Client,
    usuario_id: i32,
    nombre: &str,
) -> Result<String, Error> {
    let id = format!("{:09x}", rand::random::<u32>());
    pg_client.execute(
        "INSERT INTO portafolios (id_hex, usuario_id, nombre) VALUES ($1, $2, $3)",
        &[&id, &usuario_id, &nombre],
    )?;
    Ok(id)
}

// --- Gestión profesional de portafolios ---
#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    pub id: i32,
    pub nombre: String,
    pub id_hex: String,
}

#[tauri::command]
pub fn get_portfolios() -> Result<Vec<Portfolio>, String> {
    let mut client = connect_db()?;
    // Buscar usuario MAKIMA
    let row = client.query_opt("SELECT id FROM usuarios WHERE nombre = $1", &[&"MAKIMA"]).map_err(|e| e.to_string())?;
    let usuario_id = if let Some(row) = row {
        row.get::<_, i32>(0)
    } else {
        // Si no existe, lo crea
        let row = client.query_one("INSERT INTO usuarios (nombre) VALUES ($1) RETURNING id", &[&"MAKIMA"]).map_err(|e| e.to_string())?;
        row.get::<_, i32>(0)
    };
    let rows = client.query(
        "SELECT id, nombre, id_hex FROM portafolios WHERE usuario_id = $1 ORDER BY id",
        &[&usuario_id]
    ).map_err(|e| format!("Error fetching portfolios: {}", e))?;
    let portfolios = rows.into_iter().map(|row| Portfolio {
        id: row.get("id"),
        nombre: row.get("nombre"),
        id_hex: row.get("id_hex"),
    }).collect();
    Ok(portfolios)
}

#[tauri::command]
pub fn create_portfolio(nombre: String) -> Result<Portfolio, String> {
    println!("[create_portfolio] nombre: {} (usuario fijo: MAKIMA)", nombre);
    let mut client = connect_db()?;
    // Buscar usuario MAKIMA
    let row = client.query_opt("SELECT id FROM usuarios WHERE nombre = $1", &[&"MAKIMA"]).map_err(|e| e.to_string())?;
    let usuario_id = if let Some(row) = row {
        row.get::<_, i32>(0)
    } else {
        // Si no existe, lo crea
        let row = client.query_one("INSERT INTO usuarios (nombre) VALUES ($1) RETURNING id", &[&"MAKIMA"]).map_err(|e| e.to_string())?;
        row.get::<_, i32>(0)
    };
    println!("[create_portfolio] usuario_id de MAKIMA: {}", usuario_id);
    // Verifica si ya existe un portafolio con ese nombre para el usuario
    let exists = client.query_one(
        "SELECT 1 FROM portafolios WHERE usuario_id = $1 AND nombre = $2",
        &[&usuario_id, &nombre]
    );
    if let Ok(_) = exists {
        println!("[create_portfolio] Ya existe un portafolio con ese nombre para este usuario.");
        return Err("Ya existe un portafolio con ese nombre. Por favor, elige otro.".to_string());
    }
    let id_hex = format!("{:09x}", rand::random::<u32>());
    let row = match client.query_one(
        "INSERT INTO portafolios (id_hex, usuario_id, nombre) VALUES ($1, $2, $3) RETURNING id, nombre, id_hex",
        &[&id_hex, &usuario_id, &nombre]
    ) {
        Ok(row) => {
            println!("[create_portfolio] Portafolio creado correctamente");
            row
        },
        Err(e) => {
            println!("[create_portfolio] Error al crear portafolio: {}", e);
            return Err(e.to_string());
        }
    };
    Ok(Portfolio {
        id: row.get("id"),
        nombre: row.get("nombre"),
        id_hex: row.get("id_hex"),
    })
}

// --- Gestión de usuarios ---
#[derive(Serialize, Deserialize, Debug)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub email: Option<String>,
}

#[tauri::command]
pub fn get_users() -> Result<Vec<Usuario>, String> {
    let mut client = connect_db()?;
    let rows = client.query(
        "SELECT id, nombre, email FROM usuarios ORDER BY id",
        &[]
    ).map_err(|e| format!("Error fetching users: {}", e))?;

    let usuarios = rows.into_iter().map(|row| Usuario {
        id: row.get("id"),
        nombre: row.get("nombre"),
        email: row.get("email"),
    }).collect();

    Ok(usuarios)
}

#[tauri::command]
pub fn create_user(nombre: String, email: String) -> Result<Usuario, String> {
    let mut client = connect_db()?;
    let row = client.query_one(
        "INSERT INTO usuarios (nombre, email) VALUES ($1, $2) RETURNING id, nombre, email",
        &[&nombre, &email]
    ).map_err(|e| e.to_string())?;
    Ok(Usuario {
        id: row.get("id"),
        nombre: row.get("nombre"),
        email: row.get("email"),
    })
}