use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, ACCEPT, CONNECTION, CACHE_CONTROL};
use serde_json::Value;
use std::cmp::max;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    
    println!("╔════════════════════════════════╗");
    println!("║    search leaked data          ║");
    println!("║ enter the terms                ║");
    println!("╚════════════════════════════════╝");

    io::stdin().read_line(&mut input)?;
    let resource = input.trim();
    let url = format!("https://leakcheck.io/api/v2/query/{}", resource);
    
    let client = Client::new();
    let mut headers = HeaderMap::new();

    headers.insert(USER_AGENT, HeaderValue::from_static("PostmanRuntime/7.35.0"));
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    headers.insert("X-API-Key", HeaderValue::from_static(""));

    let response = client.get(&url).headers(headers).send()?;
    if !response.status().is_success() {
        println!("Erreur : La requête a échoué avec le statut {}", response.status());
        return Ok(());
    }

    let body = response.text()?;
    let json: Value = serde_json::from_str(&body)?;


    if let Some(found) = json.get("found").and_then(|v| v.as_u64()) {
        println!("\nNombre de résultats trouvés : {}", found);
    } else {
        println!("Erreur : Nombre de résultats introuvable.");
    }


    if let Some(entries) = json.get("result").and_then(|r| r.as_array()) {
        print_table(entries);
    } else {
        println!("Réponse inattendue : {}", json);
    }

    Ok(())
}

fn print_table(entries: &[Value]) {
    let mut max_country_len = "PAYS".len();
    let mut max_city_len = "VILLE".len();
    let mut max_username_len = "USERNAME".len();
    let mut max_email_len = "EMAIL".len();
    let mut max_breach_date_len = "DATE DE VIOLATION".len();
    let mut max_source_len = "SOURCE".len();
    let mut max_password_len = "PASSWORD".len();
    let mut max_ip_len = "IP".len();
    let mut max_first_name_len = "FIRST NAME".len();
    let mut max_last_name_len = "LAST NAME".len();
    for entry in entries.iter() {
        if let Some(obj) = entry.as_object() {
            max_country_len = max(max_country_len, obj.get("country").and_then(|v| v.as_str()).unwrap_or("N/A").len());
            max_city_len = max(max_city_len, obj.get("city").and_then(|v| v.as_str()).unwrap_or("N/A").len());
            max_username_len = max(max_username_len, obj.get("username").and_then(|v| v.as_str()).unwrap_or("N/A").len());
            max_email_len = max(max_email_len, obj.get("email").and_then(|v| v.as_str()).unwrap_or("N/A").len());
            max_breach_date_len = max(max_breach_date_len, obj.get("source").and_then(|source| source.get("breach_date")).and_then(|v| v.as_str()).unwrap_or("N/A").len());
            max_source_len = max(max_source_len, obj.get("source").and_then(|source| source.get("name")).and_then(|v| v.as_str()).unwrap_or("N/A").len());
            max_password_len = max(max_password_len, obj.get("password").and_then(|v| v.as_str()).unwrap_or("N/A").len());
            max_ip_len = max(max_ip_len, obj.get("ip").and_then(|v| v.as_str()).unwrap_or("N/A").len());
            max_first_name_len = max(max_first_name_len, obj.get("first_name").and_then(|v| v.as_str()).unwrap_or("N/A").len());
            max_last_name_len = max(max_last_name_len, obj.get("last_name").and_then(|v| v.as_str()).unwrap_or("N/A").len());
        }
    }
    let total_width = max_country_len + max_city_len + max_username_len + max_email_len + max_breach_date_len + max_source_len + max_password_len + max_ip_len + max_first_name_len + max_last_name_len + 9;
    let separator_line = "-".repeat(total_width);
    println!("{}", separator_line);
    println!(
        "{:<width$} | {:<width$} | {:<width$} | {:<width$} | {:<width$} | {:<width$} | {:<width$} | {:<width$} | {:<width$} | {:<width$}",
        pad_with_spaces("PAYS", max_country_len), 
        pad_with_spaces("VILLE", max_city_len),
        pad_with_spaces("USERNAME", max_username_len), 
        pad_with_spaces("EMAIL", max_email_len), 
        pad_with_spaces("DATE DE VIOLATION", max_breach_date_len), 
        pad_with_spaces("SOURCE", max_source_len),
        pad_with_spaces("PASSWORD", max_password_len), 
        pad_with_spaces("IP", max_ip_len), 
        pad_with_spaces("FIRST NAME", max_first_name_len), 
        pad_with_spaces("LAST NAME", max_last_name_len),
        width = max_country_len
    );
    println!("{}", separator_line);
    for entry in entries.iter() {
        if let Some(obj) = entry.as_object() {
            let country = obj.get("country").and_then(|v| v.as_str()).unwrap_or("N/A");
            let city = obj.get("city").and_then(|v| v.as_str()).unwrap_or("N/A");
            let username = obj.get("username").and_then(|v| v.as_str()).unwrap_or("N/A");
            let email = obj.get("email").and_then(|v| v.as_str()).unwrap_or("N/A");
            let breach_date = obj.get("source")
                .and_then(|source| source.get("breach_date"))
                .and_then(|v| v.as_str())
                .unwrap_or("N/A");
            let source = obj.get("source")
                .and_then(|source| source.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("N/A");
            let password = obj.get("password").and_then(|v| v.as_str()).unwrap_or("N/A");
            let ip = obj.get("ip").and_then(|v| v.as_str()).unwrap_or("N/A");
            let first_name = obj.get("first_name").and_then(|v| v.as_str()).unwrap_or("N/A");
            let last_name = obj.get("last_name").and_then(|v| v.as_str()).unwrap_or("N/A");
            println!(
                "{:<width$} | {:<width$} | {:<width$} | {:<width$} | {:<width$} | {:<width$} | {:<width$} | {:<width$} | {:<width$} | {:<width$}",
                pad_with_spaces(country, max_country_len), 
                pad_with_spaces(city, max_city_len),
                pad_with_spaces(username, max_username_len), 
                pad_with_spaces(email, max_email_len), 
                pad_with_spaces(breach_date, max_breach_date_len), 
                pad_with_spaces(source, max_source_len),
                pad_with_spaces(password, max_password_len), 
                pad_with_spaces(ip, max_ip_len), 
                pad_with_spaces(first_name, max_first_name_len), 
                pad_with_spaces(last_name, max_last_name_len),
                width = max_country_len
            );
        }
    }

    println!("{}", separator_line);
}

fn pad_with_spaces(s: &str, target_len: usize) -> String {
    let mut padded = s.to_string();
    while padded.len() < target_len {
        padded.push(' ');
    }
    padded
}
