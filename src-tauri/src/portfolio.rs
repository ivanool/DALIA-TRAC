use postgres::{Client, Error};
use rand::Rng;

fn generar_id_unico(pg_client: &mut Client) -> Result<String, Error> {
    loop {
        let id = format!("{:09x}", rand::thread_rng().gen_range(0..0x1_0000_0000));
        let rows = pg_client.query(
            "SELECT 1 FROM portafolios WHERE id_hex = $1",
            &[&id],
        )?;
        if rows.is_empty() {
            return Ok(id);
        }
    }
}

pub fn crear_portafolio(
    pg_client: &mut Client,
    usuario_id: i32,
    nombre: &str,
) -> Result<String, Error> {

    let id_hex = generar_id_unico(pg_client)?;
    pg_client.execute(
        "INSERT INTO portafolios (id_hex, usuario_id, nombre) VALUES ($1, $2, $3)",
        &[&id_hex, &usuario_id, &nombre],
    )?;
    Ok(id_hex)
}


pub fn add_ticker(
    pg_client: &mut Client,
    portfolio_id: i32,
    ticker: &String,
) -> Result<String, Error>{

    //El id del ticker es el indice que ocupa en la tabla 
    let ticker_id = 
    pg_client.execute(
        "INSERT INTO portfolio_ticker ()"
    )

    Ok()

}