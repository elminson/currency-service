use std::fs::File;
use std::io::Write;
use reqwest::blocking::get;
use chrono::{Utc};
use serde_json::json;

fn main() {
    // URL for fetching currency rates
    let url = "https://open.er-api.com/v6/latest/USD";

    // Send a GET request to the URL and get the response
    let response_text = match get(url) {
        Ok(response) => {
            match response.text() {
                Ok(text) => text,
                Err(_) => {
                    eprintln!("Error: Unable to read response body");
                    return;
                }
            }
        }
        Err(_) => {
            eprintln!("Error: Unable to fetch data from the API");
            return;
        }
    };

    // Parse the JSON response
    let json_data: serde_json::Value = match serde_json::from_str(&response_text) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Error: Unable to parse JSON response");
            return;
        }
    };

    // Extract the current date
    let current_date = Utc::now().date_naive().to_string();

    // Extract the current time
    let current_time = Utc::now().time().to_string();

    // Extract USD to EUR conversion rate
    let usd_to_eur = json_data["rates"]["EUR"].as_f64().unwrap_or(0.0);

    // Construct JSON object
    let json_obj = json!({
        "date": current_date,
        "time": current_time,
        "USD": 1,
        "EUR": usd_to_eur
    });

    // Convert JSON object to string
    let json_str = serde_json::to_string(&json_obj).unwrap();

    // Write the JSON string to a file named "usd_eur.json" in the "/tmp" directory
    let file_path = "/tmp/usd_eur.json";
    match File::create(file_path) {
        Ok(mut file) => {
            match file.write_all(json_str.as_bytes()) {
                Ok(_) => println!("Currency rates saved to {}", file_path),
                Err(_) => eprintln!("Error: Unable to write to file"),
            }
        }
        Err(_) => eprintln!("Error: Unable to create file"),
    }
}
