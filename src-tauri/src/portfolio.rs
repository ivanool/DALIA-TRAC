use postgres::{Client, Error};
use rand::Rng;
use chrono::NaiveDateTime;

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