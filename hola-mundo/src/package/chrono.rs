use chrono::{prelude::*, Duration};
fn main() {
    // Fecha hora UTC
    let ahora_utc : DateTime<Utc> = Utc::now();
    println!("Fecha hora en UTC: {}", ahora_utc);

    // Fecha hora local
    let ahora_local: DateTime<Local> = Local::now();
    println!("Fecha/hora Local: {}", ahora_local);

    //  
    let ahora = Local::now();
    let formateado = ahora.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Fecha / Hora formateado: {}", formateado);

    let entrada = "2023-01-20 12:34:56";
    let formato = "%Y-%m-%d %H:%M:%S";

    let naive_dt = NaiveDateTime::parse_from_str(entrada, formato)
        .expect("Error parseado fecha/hora");
    println!("NaiveDateTime : {}", naive_dt);

    let now = Utc::now();
    println!("Now UTC: {}", now);

    let a_week = now + Duration::days(7);
    println!("A week: {}", a_week);

    let two_hours_ago = now - Duration::hours(2);
    println!("Two hours ago: {}", two_hours_ago);


}
/*
    %Y -> Year
    %m -> Mount
    %d -> Day
    %H -> Hour
    %M -> Minutes
    %S -> Secomds
*/