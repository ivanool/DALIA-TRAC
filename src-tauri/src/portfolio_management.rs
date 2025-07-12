use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use postgres::{Client, NoTls, Transaction};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetTransaction {
    pub id: i32,
    pub portfolio_id: i32,
    pub ticker: String,
    pub transaction_type: String,
    pub quantity: f64,
    pub price: f64,
    pub transaction_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CashFlow {
    pub id: i32,
    pub portfolio_id: i32,
    pub flow_type: String,
    pub amount: f64,
    pub flow_date: NaiveDate,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionSlot {
    pub ticker: String,
    pub total_quantity: f64,
    pub average_price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfitLoss {
    pub ticker: String,
    pub total_quantity: f64,
    pub average_price: f64,
    pub current_price: f64,
    pub unrealized_pl: f64,
    pub unrealized_pl_percent: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dividend {
    pub ticker: String,
    pub total_dividend_amount: f64,
    pub dividend_date: NaiveDate,
}

fn connect_db() -> Result<Client, String> {
    Client::connect("host=localhost user=garden_admin password=password dbname=dalia_db", NoTls)
        .map_err(|e| format!("DB connection error: {}", e))
}

#[tauri::command]
pub fn add_cash_movement(
    portfolio_id: i32,
    flow_type: String,
    amount: f64,
    flow_date: NaiveDate,
    description: String,
) -> Result<CashFlow, String> {
    if flow_type != "deposit" && flow_type != "withdrawal" {
        return Err("flow_type debe ser 'deposit' o 'withdrawal'".to_string());
    }
    let mut client = connect_db()?;
    let row = client.query_one(
        "INSERT INTO cashflow (portfolio_id, flow_type, amount, flow_date, description) VALUES ($1, $2, $3, $4, $5) RETURNING id, portfolio_id, flow_type, amount, flow_date, description",
        &[&portfolio_id, &flow_type, &amount, &flow_date, &Some(description.clone())]
    ).map_err(|e| e.to_string())?;
    Ok(CashFlow {
        id: row.get("id"),
        portfolio_id: row.get("portfolio_id"),
        flow_type: row.get("flow_type"),
        amount: row.get("amount"),
        flow_date: row.get("flow_date"),
        description: row.get("description"),
    })
}

#[tauri::command]
pub fn get_cash_balance(portfolio_id: i32) -> Result<f64, String> {
    let mut client = connect_db()?;
    let rows = client.query(
        "SELECT flow_type, amount FROM cashflow WHERE portfolio_id = $1",
        &[&portfolio_id]
    ).map_err(|e| e.to_string())?;

    let mut balance = 0.0;
    for row in rows {
        let flow_type: String = row.get("flow_type");
        let amount: f64 = row.get("amount");
        match flow_type.as_str() {
            "deposit" | "sell_proceeds" | "dividend" => balance += amount,
            "withdrawal" | "buy_cost" => balance += amount, // buy_cost y withdrawal deben ser negativos
            _ => {}
        }
    }
    Ok(balance)
}

#[tauri::command]
pub fn get_cash_flow_history(portfolio_id: i32) -> Result<Vec<CashFlow>, String> {
    let mut client = connect_db()?;
    let rows = client.query(
        "SELECT id, portfolio_id, flow_type, amount, flow_date, description FROM cashflow WHERE portfolio_id = $1 ORDER BY flow_date",
        &[&portfolio_id]
    ).map_err(|e| e.to_string())?;

    let history = rows.into_iter().map(|row| CashFlow {
        id: row.get("id"),
        portfolio_id: row.get("portfolio_id"),
        flow_type: row.get("flow_type"),
        amount: row.get("amount"),
        flow_date: row.get("flow_date"),
        description: row.get("description"),
    }).collect();

    Ok(history)
}

#[tauri::command]
pub fn add_asset_transaction(
    portfolio_id: i32,
    ticker: String,
    transaction_type: String, // "buy" o "sell"
    quantity: f64,
    price: f64,
    transaction_date: NaiveDate,
    use_cash_from_portfolio: bool,
) -> Result<AssetTransaction, String> {
    if transaction_type != "buy" && transaction_type != "sell" {
        return Err("transaction_type debe ser 'buy' o 'sell'".to_string());
    }
    let mut client = connect_db()?;
    let mut tx = client.transaction().map_err(|e| e.to_string())?;
    let total_cost = quantity * price;

    if use_cash_from_portfolio {
        let balance = get_cash_balance_tx(&mut tx, portfolio_id)?;
        if transaction_type == "buy" && balance < total_cost {
            return Err("Saldo insuficiente para realizar la compra".to_string());
        }
    }

    let row = tx.query_one(
        "INSERT INTO portfolio_transactions (portfolio_id, ticker, transaction_type, quantity, price, transaction_date) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, portfolio_id, ticker, transaction_type, quantity, price, transaction_date",
        &[&portfolio_id, &ticker, &transaction_type, &quantity, &price, &transaction_date]
    ).map_err(|e| e.to_string())?;

    if use_cash_from_portfolio {
        let (flow_type, amount) = match transaction_type.as_str() {
            "buy" => ("buy_cost", -total_cost),
            "sell" => ("sell_proceeds", total_cost),
            _ => unreachable!(),
        };
        tx.execute(
            "INSERT INTO cashflow (portfolio_id, flow_type, amount, flow_date, description) VALUES ($1, $2, $3, $4, $5)",
            &[&portfolio_id, &flow_type, &amount, &transaction_date, &Some(format!("{} {}", transaction_type, ticker))]
        ).map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(AssetTransaction {
        id: row.get("id"),
        portfolio_id: row.get("portfolio_id"),
        ticker: row.get("ticker"),
        transaction_type: row.get("transaction_type"),
        quantity: row.get("quantity"),
        price: row.get("price"),
        transaction_date: row.get("transaction_date"),
    })
}

fn get_cash_balance_tx(tx: &mut Transaction, portfolio_id: i32) -> Result<f64, String> {
    let rows = tx.query(
        "SELECT flow_type, amount FROM cashflow WHERE portfolio_id = $1",
        &[&portfolio_id]
    ).map_err(|e| e.to_string())?;

    let mut balance = 0.0;
    for row in rows {
        let flow_type: String = row.get("flow_type");
        let amount: f64 = row.get("amount");
        match flow_type.as_str() {
            "deposit" | "sell_proceeds" | "dividend" => balance += amount,
            "withdrawal" | "buy_cost" => balance += amount,
            _ => {}
        }
    }
    Ok(balance)
}

#[tauri::command]
pub fn delete_asset_transaction(transaction_id: i32) -> Result<String, String> {
    let mut client = connect_db()?;
    let n = client.execute(
        "DELETE FROM portfolio_transactions WHERE id = $1",
        &[&transaction_id]
    ).map_err(|e| e.to_string())?;
    if n == 1 {
        Ok("Transacción eliminada correctamente. Recuerda ajustar el cashflow manualmente si es necesario.".to_string())
    } else {
        Err("No se encontró la transacción para eliminar.".to_string())
    }
}

#[tauri::command]
pub fn get_portfolio_slots(portfolio_id: i32) -> Result<Vec<PositionSlot>, String> {
    let mut client = connect_db()?;
    let rows = client.query(
        "SELECT ticker, transaction_type, quantity, price FROM portfolio_transactions WHERE portfolio_id = $1",
        &[&portfolio_id]
    ).map_err(|e| e.to_string())?;

    let mut positions: HashMap<String, (f64, f64)> = HashMap::new();
    for row in rows {
        let ticker: String = row.get("ticker");
        let transaction_type: String = row.get("transaction_type");
        let quantity: f64 = row.get("quantity");
        let price: f64 = row.get("price");
        let entry = positions.entry(ticker.clone()).or_insert((0.0, 0.0));
        match transaction_type.as_str() {
            "buy" => {
                entry.0 += quantity;
                entry.1 += quantity * price;
            }
            "sell" => {
                entry.0 -= quantity;
                entry.1 -= quantity * price;
            }
            _ => {}
        }
    }

    let slots = positions.into_iter().filter_map(|(ticker, (qty, cost))| {
        if qty.abs() > 1e-6 {
            Some(PositionSlot {
                ticker,
                total_quantity: qty,
                average_price: if qty.abs() > 1e-6 { cost / qty } else { 0.0 },
            })
        } else {
            None
        }
    }).collect();

    Ok(slots)
}

#[tauri::command]
pub fn calculate_portfolio_pl(portfolio_id: i32) -> Result<Vec<ProfitLoss>, String> {
    let slots = get_portfolio_slots(portfolio_id)?;
    let mut result = Vec::new();
    for slot in slots {
        let current_price = get_current_price(&slot.ticker);
        let unrealized_pl = (current_price - slot.average_price) * slot.total_quantity;
        let unrealized_pl_percent = if slot.average_price.abs() > 1e-6 {
            (unrealized_pl / (slot.average_price * slot.total_quantity)) * 100.0
        } else {
            0.0
        };
        result.push(ProfitLoss {
            ticker: slot.ticker,
            total_quantity: slot.total_quantity,
            average_price: slot.average_price,
            current_price,
            unrealized_pl,
            unrealized_pl_percent,
        });
    }
    Ok(result)
}

#[tauri::command]
pub fn register_dividend_as_cash(
    portfolio_id: i32,
    ticker: String,
    total_dividend_amount: f64,
    dividend_date: NaiveDate,
) -> Result<CashFlow, String> {
    let mut client = connect_db()?;
    let row = client.query_one(
        "INSERT INTO cashflow (portfolio_id, flow_type, amount, flow_date, description) VALUES ($1, $2, $3, $4, $5) RETURNING id, portfolio_id, flow_type, amount, flow_date, description",
        &[&portfolio_id, &"dividend", &total_dividend_amount, &dividend_date, &Some(format!("Dividendo de {}", ticker))]
    ).map_err(|e| e.to_string())?;
    Ok(CashFlow {
        id: row.get("id"),
        portfolio_id: row.get("portfolio_id"),
        flow_type: row.get("flow_type"),
        amount: row.get("amount"),
        flow_date: row.get("flow_date"),
        description: row.get("description"),
    })
}

fn get_current_price(_ticker: &str) -> f64 {
    100.0
}

fn get_dividend_per_share(_ticker: &str) -> f64 {
    2.5
}
