use csv::Reader;
use std::collections::HashMap;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: csv-cli <path_to_csv_file>");
        return Ok(());
    }

    let csv_file_path = &args[1];

    let records = read_csv_records(csv_file_path)?;

    let (average, min, max) = analyse_data(records, String::from("ND"))?;
    println!("Min: {}, Max: {}, Avg: {}", min, max, average);

    Ok(())
}

fn read_csv_records(csv_file_path: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    println!("Loading CSV file {} in memory.", csv_file_path);

    let mut rdr = Reader::from_path(csv_file_path)?;

    let headers = rdr.headers()?.clone();
    let mut records: Vec<HashMap<String, String>> = Vec::new();

    for result in rdr.records() {
        let record = result?;

        let mut record_map = HashMap::new();
        for (header, value) in headers.iter().zip(record.iter()) {
            record_map.insert(header.to_string(), value.to_string());
        }

        records.push(record_map);
    }

    Ok(records)
}

fn analyse_data(
    data: Vec<HashMap<String, String>>,
    col: String,
) -> Result<(f64, f64, f64), Box<dyn Error>> {
    println!("Analysing data...");
    let data_count = data.len() as f64;
    let mut max: f64 = 0.0;
    let mut min: f64 = 0.0;
    let mut sum: f64 = 0.0;

    for record in data.iter() {
        // This implementation handles bad data in a given record and skips it.
        // The previous implementation would throw an error instead and exit, not the right behaviour!
        if let Some(value) = record.get(&col) {
            if let Ok(national_demand) = value.parse::<f64>() {
                if max < national_demand {
                    max = national_demand;
                }

                if min > national_demand || min == 0.0 {
                    min = national_demand;
                }

                sum += national_demand as f64;
            } else {
                println!("Warning: Invalid value encountered: {}", value);
            }
        }
    }

    let average: f64 = sum / data_count;

    Ok((average, min, max))
}
