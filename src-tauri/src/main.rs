#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use postgres::{Client, NoTls};
mod get_data;
use get_data::get_intradia;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::connect(
        "host=localhost user=garden_admin password=password dbname=dalia_db",
        NoTls,
    )?;

    //get_data::get_ticker(&mut client)?;

    
    let _parse_json = |s: Option<String>| -> Option<serde_json::Value> {
        s.and_then(|json_str| serde_json::from_str(&json_str).ok())
    };
    let emisoras = ["KOFUBL", "FEMSAUBD", "MEDICAB", "AMXB"];
    let inicio = "2025-06-17";
    let fin = "2025-06-17";
    //show_data(&mut client)?;
    get_intradia(&emisoras, &inicio, &fin, &mut client)?;
    dalia_trac_lib::run()
    Ok(())
}