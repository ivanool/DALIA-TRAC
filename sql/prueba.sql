let row = pg_client.query_opt(
    "SELECT precio, fecha_hora
     FROM intradia_data
     WHERE emisora = $1
       AND fecha_hora < $2
     ORDER BY fecha_hora DESC
     LIMIT 1",
    &[&emisora, &fecha_actual],
)?;
let precio_ayer = row.map(|r| r.get::<_, f64>(0));
let fecha_ayer = row.map(|r| r.get::<_, chrono::NaiveDateTime>(1));
println!("Precio anterior encontrado: {:?} en {:?}", precio_ayer, fecha_ayer);