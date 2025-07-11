use postgres::{Client, Error};
use rand::Rng;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use tauri::command;
use std::collections::HashMap;

fn gen_id(pg_client: &mut Client) -> Result<String, Error> {
    loop {
        let id = format!("{:09x}", rand::thread_rng().gen_range(0u64..0x1_0000_0000u64));
        let rows = pg_client.query(
            "SELECT 1 FROM portafolios WHERE id_hex = $1",
            &[&id],
        )?;
        if rows.is_empty() {
            return Ok(id);
        }
    }
}

pub fn add_portfolio(
    pg_client: &mut Client,
    usuario_id: i32,
    nombre: &str,
) -> Result<String, Error> {

    let id_hex = gen_id(pg_client)?;
    pg_client.execute(
        "INSERT INTO portafolios (id_hex, usuario_id, nombre) VALUES ($1, $2, $3)",
        &[&id_hex, &usuario_id, &nombre],
    )?;
    Ok(id_hex)
}

pub fn delete_portfolio(pg_client: &mut Client, id_hex: &str) -> Result<u64, Error> {
    let rows_deleted = pg_client.execute(
        "DELETE FROM portafolios WHERE id_hex = $1",
        &[&id_hex],
    )?;
    Ok(rows_deleted)
}

pub fn list_portfolios(pg_client: &mut Client, usuario_id: i32) -> Result<Vec<(i32, String, String)>, Error> {
    let rows = pg_client.query(
        "SELECT id, id_hex, nombre FROM portafolios WHERE usuario_id = $1 ORDER BY created_at DESC",
        &[&usuario_id],
    )?;
    let mut portfolios = Vec::new();
    for row in rows {
        let id: i32 = row.get(0);
        let id_hex: String = row.get(1);
        let nombre: String = row.get(2);
        portfolios.push((id, id_hex, nombre));
    }
    Ok(portfolios)
}

pub fn add_ticker(
    pg_client: &mut Client,
    portafolio_id: i32,
    ticker: &str,
    emisoras: &str,
    serie: &str,
    fecha: Option<chrono::DateTime<chrono::Utc>>, // CAMBIO DE TIPO
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


pub fn remove_ticker(pg_client: &mut Client, id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let rows_deleted = pg_client.execute(
        "DELETE FROM portafolio_ticker WHERE id = $1",
        &[&id],
    )?;
    Ok(())
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

pub fn add_dividendo(
    pg_client: &mut Client,
    portafolio_ticker_id: i32,
    monto: f64,
    fecha: Option<chrono::NaiveDateTime>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(fecha) = fecha {
        pg_client.execute(
            "INSERT INTO dividendos (portafolio_ticker_id, monto, fecha) VALUES ($1, $2, $3)",
            &[&portafolio_ticker_id, &monto, &fecha],
        )?;
    } else {
        pg_client.execute(
            "INSERT INTO dividendos (portafolio_ticker_id, monto) VALUES ($1, $2)",
            &[&portafolio_ticker_id, &monto],
        )?;
    }
    Ok(())
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

pub fn list_dividendos_by_portafolio(
    pg_client: &mut Client,
    portafolio_id: i32,
) -> Result<Vec<(i32, i32, f64, chrono::NaiveDateTime)>, Box<dyn std::error::Error>> {
    let rows = pg_client.query(
        "SELECT d.id, d.portafolio_ticker_id, d.monto, d.fecha
         FROM dividendos d
         JOIN portafolio_ticker pt ON d.portafolio_ticker_id = pt.id
         WHERE pt.portafolio_id = $1
         ORDER BY d.fecha DESC",
        &[&portafolio_id],
    )?;
    let mut dividendos = Vec::new();
    for row in rows {
        let id: i32 = row.get(0);
        let portafolio_ticker_id: i32 = row.get(1);
        let monto: f64 = row.get(2);
        let fecha: chrono::NaiveDateTime = row.get(3);
        dividendos.push((id, portafolio_ticker_id, monto, fecha));
    }
    Ok(dividendos)
}


pub fn add_cashflow(
    pg_client: &mut Client,
    portafolio_id: i32,
    monto: f64,
    tipo: &str,
    descripcion: Option<&str>,
    fecha: Option<NaiveDateTime>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(fecha) = fecha {
        pg_client.execute(
            "INSERT INTO cashflow (portafolio_id, monto, tipo, fecha, descripcion) VALUES ($1, $2, $3, $4, $5)",
            &[&portafolio_id, &monto, &tipo, &fecha, &descripcion],
        )?;
    } else {
        pg_client.execute(
            "INSERT INTO cashflow (portafolio_id, monto, tipo, descripcion) VALUES ($1, $2, $3, $4)",
            &[&portafolio_id, &monto, &tipo, &descripcion],
        )?;
    }
    Ok(())
}

pub fn list_cashflow(
    pg_client: &mut Client,
    portafolio_id: i32,
) -> Result<Vec<(i32, f64, String, NaiveDateTime, Option<String>)>, Box<dyn std::error::Error>> {
    let rows = pg_client.query(
        "SELECT id, monto, tipo, fecha, descripcion FROM cashflow WHERE portafolio_id = $1 ORDER BY fecha DESC",
        &[&portafolio_id],
    )?;
    let mut movimientos = Vec::new();
    for row in rows {
        let id: i32 = row.get(0);
        let monto: f64 = row.get(1);
        let tipo: String = row.get(2);
        let fecha: NaiveDateTime = row.get(3);
        let descripcion: Option<String> = row.get(4);
        movimientos.push((id, monto, tipo, fecha, descripcion));
    }
    Ok(movimientos)
}

pub fn get_cashflow_balance(
    pg_client: &mut Client,
    portafolio_id: i32,
) -> Result<f64, Box<dyn std::error::Error>> {
    let row = pg_client.query_one(
        "SELECT COALESCE(SUM(monto), 0) FROM cashflow WHERE portafolio_id = $1",
        &[&portafolio_id],
    )?;
    let saldo: f64 = row.get(0);
    Ok(saldo)
}

pub fn filter_cashflow(
    pg_client: &mut Client,
    portafolio_id: i32,
    tipo: Option<&str>,
    fecha_inicio: Option<NaiveDateTime>,
    fecha_fin: Option<NaiveDateTime>,
) -> Result<Vec<(i32, f64, String, NaiveDateTime, Option<String>)>, Box<dyn std::error::Error>> {
    let mut query = String::from("SELECT id, monto, tipo, fecha, descripcion FROM cashflow WHERE portafolio_id = $1");
    let mut params: Vec<&(dyn postgres::types::ToSql + Sync)> = vec![&portafolio_id];
    let mut param_idx = 2;
    let mut tipo_owned: Option<String> = tipo.map(|s| s.to_string());
    let mut fecha_inicio_owned: Option<NaiveDateTime> = fecha_inicio;
    let mut fecha_fin_owned: Option<NaiveDateTime> = fecha_fin;

    if tipo_owned.is_some() {
        query.push_str(&format!(" AND tipo = ${}", param_idx));
        param_idx += 1;
    }
    if fecha_inicio_owned.is_some() {
        query.push_str(&format!(" AND fecha >= ${}", param_idx));
        param_idx += 1;
    }
    if fecha_fin_owned.is_some() {
        query.push_str(&format!(" AND fecha <= ${}", param_idx));
        param_idx += 1;
    }
    query.push_str(" ORDER BY fecha DESC");

    if let Some(ref tipo_val) = tipo_owned {
        params.push(tipo_val);
    }
    if let Some(ref fecha_inicio_val) = fecha_inicio_owned {
        params.push(fecha_inicio_val);
    }
    if let Some(ref fecha_fin_val) = fecha_fin_owned {
        params.push(fecha_fin_val);
    }

    let rows = pg_client.query(&query, &params)?;
    let mut movimientos = Vec::new();
    for row in rows {
        let id: i32 = row.get(0);
        let monto: f64 = row.get(1);
        let tipo: String = row.get(2);
        let fecha: NaiveDateTime = row.get(3);
        let descripcion: Option<String> = row.get(4);
        movimientos.push((id, monto, tipo, fecha, descripcion));
    }
    Ok(movimientos)
}

pub fn delete_cashflow(
    pg_client: &mut Client,
    cashflow_id: i32,
) -> Result<u64, Box<dyn std::error::Error>> {
    let rows_deleted = pg_client.execute(
        "DELETE FROM cashflow WHERE id = $1",
        &[&cashflow_id],
    )?;
    Ok(rows_deleted)
}

#[derive(Serialize, Deserialize)]
pub struct PortfolioTransaction {
    pub transaction_id: i32,
    pub portfolio_id: i32,
    pub user_id: i32,
    pub ticker: String,
    pub transaction_type: String,
    pub quantity: f64,
    pub price: Option<f64>,
    pub transaction_date: chrono::DateTime<chrono::Utc>,
    pub total_amount: f64,
    pub currency: String,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct PortfolioHolding {
    pub ticker: String,
    pub quantity: f64,
    pub avg_cost: f64,
    pub market_price: Option<f64>,
    pub market_value: Option<f64>,
    pub unrealized_pl: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct PortfolioSummary {
    pub holdings: Vec<PortfolioHolding>,
    pub cash_balance: f64,
    pub total_value: f64,
    pub total_unrealized_pl: f64,
    pub total_realized_pl: f64,
}

#[command]
pub fn add_portfolio_transaction(
    portfolio_id: i32,
    user_id: i32,
    ticker: String,
    transaction_type: String,
    quantity: f64,
    price: Option<f64>,
    transaction_date: String,
    total_amount: f64,
    currency: String,
    notes: Option<String>,
) -> Result<(), String> {
    let mut client = Client::connect(
        "host=localhost user=garden_admin password=password dbname=dalia_db",
        postgres::NoTls,
    ).map_err(|e| e.to_string())?;
    let _ = client.execute(
        "INSERT INTO portfolio_transactions (portfolio_id, user_id, ticker, transaction_type, quantity, price, transaction_date, total_amount, currency, notes) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
        &[&portfolio_id, &user_id, &ticker, &transaction_type, &quantity, &price, &transaction_date, &total_amount, &currency, &notes],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn get_portfolio_summary(portfolio_id: i32, user_id: i32) -> Result<PortfolioSummary, String> {
    let mut client = Client::connect(
        "host=localhost user=garden_admin password=password dbname=dalia_db",
        postgres::NoTls,
    ).map_err(|e| e.to_string())?;
    // 1. Obtener todas las transacciones del portafolio
    let rows = client.query(
        "SELECT ticker, transaction_type, quantity, price, total_amount FROM portfolio_transactions WHERE portfolio_id = $1 AND user_id = $2 ORDER BY transaction_date ASC",
        &[&portfolio_id, &user_id],
    ).map_err(|e| e.to_string())?;
    let mut holdings: HashMap<String, (f64, f64)> = HashMap::new(); // ticker -> (cantidad, costo total acumulado)
    let mut realized_pl = 0.0;
    for row in &rows {
        let ticker: String = row.get(0);
        let ttype: String = row.get(1);
        let qty: f64 = row.get(2);
        let price: Option<f64> = row.get(3);
        let total: f64 = row.get(4);
        let entry = holdings.entry(ticker.clone()).or_insert((0.0, 0.0));
        match ttype.as_str() {
            "BUY" => {
                entry.0 += qty;
                entry.1 += total;
            },
            "SELL" => {
                let avg_cost = if entry.0.abs() > 1e-8 { entry.1 / entry.0 } else { 0.0 };
                realized_pl += total - avg_cost * qty;
                entry.0 -= qty;
                entry.1 -= avg_cost * qty;
            },
            _ => {}
        }
    }
    // 2. Obtener precios de mercado (stub: None)
    let mut holdings_vec = Vec::new();
    let mut total_value = 0.0;
    let mut total_unrealized = 0.0;
    for (ticker, (qty, cost)) in holdings.iter() {
        if qty.abs() < 1e-8 { continue; }
        let avg_cost = if *qty != 0.0 { cost / qty } else { 0.0 };
        let market_price = None; // Aquí deberías consultar una API de precios
        let market_value = market_price.map(|p| p * qty);
        let unrealized = market_value.map(|mv| mv - cost);
        if let Some(mv) = market_value { total_value += mv; }
        if let Some(u) = unrealized { total_unrealized += u; }
        holdings_vec.push(PortfolioHolding {
            ticker: ticker.clone(),
            quantity: *qty,
            avg_cost,
            market_price,
            market_value,
            unrealized_pl: unrealized,
        });
    }
    // 3. Obtener efectivo
    let cash_row = client.query_opt(
        "SELECT balance FROM portfolio_cash WHERE portfolio_id = $1 AND user_id = $2",
        &[&portfolio_id, &user_id],
    ).map_err(|e| e.to_string())?;
    let cash_balance = cash_row.map(|r| r.get::<_, f64>(0)).unwrap_or(0.0);
    let total = total_value + cash_balance;
    Ok(PortfolioSummary {
        holdings: holdings_vec,
        cash_balance,
        total_value: total,
        total_unrealized_pl: total_unrealized,
        total_realized_pl: realized_pl,
    })
}

#[command]
pub fn get_cash_flow_history(portfolio_id: i32, user_id: i32) -> Result<Vec<(String, f64)>, String> {
    let mut client = Client::connect(
        "host=localhost user=garden_admin password=password dbname=dalia_db",
        postgres::NoTls,
    ).map_err(|e| e.to_string())?;
    let rows = client.query(
        "SELECT transaction_date, total_amount, transaction_type FROM portfolio_transactions WHERE portfolio_id = $1 AND user_id = $2 ORDER BY transaction_date ASC",
        &[&portfolio_id, &user_id],
    ).map_err(|e| e.to_string())?;
    let mut history = Vec::new();
    let mut balance = 0.0;
    for row in rows {
        let date: chrono::DateTime<chrono::Utc> = row.get(0);
        let amount: f64 = row.get(1);
        let ttype: String = row.get(2);
        match ttype.as_str() {
            "BUY" => balance -= amount,
            "SELL" | "DIVIDEND" | "DEPOSIT" => balance += amount,
            "WITHDRAWAL" => balance -= amount,
            _ => {}
        }
        history.push((date.to_rfc3339(), balance));
    }
    Ok(history)
}

#[command]
pub fn get_dividend_history(portfolio_id: i32, user_id: i32) -> Result<Vec<PortfolioTransaction>, String> {
    let mut client = Client::connect(
        "host=localhost user=garden_admin password=password dbname=dalia_db",
        postgres::NoTls,
    ).map_err(|e| e.to_string())?;
    let rows = client.query(
        "SELECT transaction_id, portfolio_id, user_id, ticker, transaction_type, quantity, price, transaction_date, total_amount, currency, notes, created_at, updated_at FROM portfolio_transactions WHERE portfolio_id = $1 AND user_id = $2 AND transaction_type = 'DIVIDEND' ORDER BY transaction_date DESC",
        &[&portfolio_id, &user_id],
    ).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for row in rows {
        result.push(PortfolioTransaction {
            transaction_id: row.get(0),
            portfolio_id: row.get(1),
            user_id: row.get(2),
            ticker: row.get(3),
            transaction_type: row.get(4),
            quantity: row.get(5),
            price: row.get(6),
            transaction_date: row.get(7),
            total_amount: row.get(8),
            currency: row.get(9),
            notes: row.get(10),
            created_at: row.get(11),
            updated_at: row.get(12),
        });
    }
    Ok(result)
}