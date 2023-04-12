use csv::Reader;
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: csv-cli <path_to_csv_file>");
        return;
    }

    let csv_file_path = &args[1];
    println!("Analysing csv file: {}", csv_file_path);

    let mut rdr = Reader::from_path(csv_file_path).expect("File does not exist");

    let headers = rdr.headers().expect("no headers.").clone();
    let mut records: Vec<HashMap<String, String>> = Vec::new();

    for result in rdr.records() {
        let record = result.expect("no record");

        let mut record_map = HashMap::new();
        for (header, value) in headers.iter().zip(record.iter()) {
            record_map.insert(header.to_string(), value.to_string());
        }

        records.push(record_map);
    }

    analyse_data(records, String::from("TSD"));
}

fn analyse_data(data: Vec<HashMap<String, String>>, col: String) {
    let data_count = data.len();
    let mut max = 0;
    let mut min = 0;
    let mut sum: f64 = 0.0;

    for record in data.iter() {
        let national_demand = record
            .get(&col)
            .expect("Not a valid key")
            .parse()
            .expect("Not a valid number");
        if max < national_demand {
            max = national_demand;
        }

        if min > national_demand || min == 0 {
            min = national_demand;
        }

        sum += national_demand as f64;
    }

    let average: f64 = sum / (data_count as f64);

    println!("Min: {}, Max: {}, Avg: {}", min, max, average);
}
