use std::collections::HashSet;
use postgres::{Client, NoTls};

pub fn eliminar_duplicados_isin(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Buscar todos los isins duplicados
    let rows = client.query(
        "SELECT isin FROM emisoras WHERE isin IS NOT NULL GROUP BY isin HAVING COUNT(*) > 1",
        &[],
    )?;

    for row in rows {
        let isin: String = row.get(0);
        // 2. Obtener los ctid de todas las filas con ese isin
        let ctids = client.query(
            "SELECT ctid FROM emisoras WHERE isin = $1",
            &[&isin],
        )?;
        // 3. Mantener solo el primer ctid, borrar los demás
        if ctids.len() > 1 {
            for ctid_row in ctids.iter().skip(1) {
                let ctid: String = ctid_row.get(0);
                client.execute(
                    &format!("DELETE FROM emisoras WHERE ctid = '{}'", ctid),
                    &[],
                )?;
            }
        }
    }
    println!("Duplicados de ISIN eliminados.");
    Ok(())
}

pub fn eliminar_duplicados_emisoras_iterativo(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    // Obtener todos los registros ordenados por emisoras, serie, isin
    let rows = client.query(
        "SELECT ctid, emisoras, serie, isin FROM emisoras ORDER BY emisoras, serie, isin",
        &[],
    )?;
    let mut prev_emisoras = String::new();
    let mut prev_serie = String::new();
    let mut prev_isin = String::new();
    let mut first = true;
    for row in &rows {
        let ctid: String = row.get(0);
        let emisoras: String = row.get(1);
        let serie: String = row.get(2);
        let isin: String = row.get(3);
        if !first && emisoras == prev_emisoras && serie == prev_serie && isin == prev_isin {
            // Duplicado, borrar
            client.execute(&format!("DELETE FROM emisoras WHERE ctid = '{}'", ctid), &[])?;
        } else {
            prev_emisoras = emisoras;
            prev_serie = serie;
            prev_isin = isin;
            first = false;
        }
    }
    println!("Duplicados exactos de emisoras/serie/isin eliminados.");
    Ok(())
}

pub fn eliminar_duplicados_emisoras_hashset(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    let rows = client.query(
        "SELECT ctid, emisoras, serie, isin FROM emisoras",
        &[],
    )?;
    let mut vistos = HashSet::new();
    for row in &rows {
        let ctid: String = row.get(0);
        let emisoras: String = row.get(1);
        let serie: String = row.get(2);
        let isin: String = row.get(3);
        let clave = format!("{}|{}|{}", emisoras.trim(), serie.trim(), isin.trim());
        if vistos.contains(&clave) {
            client.execute(
                &format!("DELETE FROM emisoras WHERE ctid = '{}'", ctid),
                &[],
            )?;
        } else {
            vistos.insert(clave);
        }
    }
    println!("Duplicados exactos de emisoras/serie/isin eliminados (HashSet).");
    Ok(())
}

pub fn eliminar_duplicados_isin_hashset(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let rows = client.query(
            "SELECT ctid::text, isin FROM emisoras WHERE isin IS NOT NULL AND TRIM(isin) <> ''",
            &[],
        )?;
        let mut vistos = HashSet::new();
        let mut eliminados = 0;
        for row in &rows {
            let ctid: String = row.get(0);
            let isin: String = row.get(1);
            let clave = isin.trim().to_uppercase();
            if vistos.contains(&clave) {
                client.execute(
                    "DELETE FROM emisoras WHERE ctid::text = $1",
                    &[&ctid],
                )?;
                eliminados += 1;
            } else {
                vistos.insert(clave);
            }
        }
        if eliminados == 0 {
            break;
        }
    }
    println!("Duplicados exactos de ISIN eliminados (HashSet, robusto). Solo se borra si ISIN es igual y no nulo.");
    Ok(())
}
