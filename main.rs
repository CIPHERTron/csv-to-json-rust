extern crate csv;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::io;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct CsvRecord {
    // Define your CSV structure here based on your data
    address: String,
    city: String,
    companyName: String,
    contactName: String,
    contactTitle: String,
    country: String,
    customerID: String,
    fax: String,
    phone: String,
    postalCode: String,
    region: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file = File::open("input.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    // Collect CSV data and convert to JSON
    let mut json_data: Vec<Value> = Vec::new();

    for result in rdr.deserialize() {
        let record: CsvRecord = result?;
        let json_entry = serde_json::to_value(record)?;
        json_data.push(json_entry);
    }

    // Serialize JSON data
    let json_string = serde_json::to_string_pretty(&json_data)?;

    // Write JSON to a file
    let mut json_file = File::create("output.json")?;
    json_file.write_all(json_string.as_bytes())?;

    println!("CSV to JSON conversion successful. JSON data written to output.json");

    Ok(())
}
