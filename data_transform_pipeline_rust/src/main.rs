use std::error::Error;
use std::fs::File;
use std::process;

use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    name: String,
    age: String,
    city: String,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // Read input file
    let file = File::open("data/input.csv")?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    // Write output file
    let output = File::create("data/output.csv")?;
    let mut wtr = WriterBuilder::new().from_writer(output);

    for result in rdr.deserialize() {
        let record: Record = result?;

        // Skip rows with missing values
        if record.name.trim().is_empty() || record.age.trim().is_empty() || record.city.trim().is_empty() {
            continue;
        }

        // Optional: Normalize name (capitalize)
        let cleaned_record = Record {
            name: capitalize(&record.name),
            age: record.age.trim().to_string(),
            city: record.city.trim().to_string(),
        };

        wtr.serialize(cleaned_record)?;
    }

    wtr.flush()?;
    println!("✅ Data transformation complete! Output saved to data/output.csv");

    Ok(())
}

fn capitalize(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
