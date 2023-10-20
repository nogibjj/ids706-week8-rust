extern crate csv;
extern crate nalgebra as na;

use std::error::Error;
use std::time::Instant;
use na::DVector;

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    // Sample data as a string
    let data_csv = "
        Id,SepalLengthCm
        1,5.1
        2,4.9
        3,4.7
        4,4.6
        5,5.0
        6,5.4
        7,4.6
    ";

    // Set up the CSV reader
    let mut rdr = csv::Reader::from_reader(data_csv.as_bytes());
    
    let mut rows: Vec<f64> = Vec::new();

    // Read and collect all rows into the rows Vec
    for result in rdr.deserialize() {
        let (_id, length): (usize, f64) = result?;
        rows.push(length);
    }

    let data = DVector::from_vec(rows);

    // Compute statistics
    let mean = data.mean();
    let mut sorted: Vec<f64> = data.clone().data.into();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = sorted.len() / 2;
    let median = if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    };
    let std_dev = {
        let mean = data.mean();
        f64::sqrt(data.iter().map(|&x| (x - mean) * (x - mean)).sum::<f64>() / data.len() as f64)
    };

    // Print statistics
    println!("Mean of SepalLengthCm = {}", mean);
    println!("Median of SepalLengthCm = {}", median);
    println!("Std Dev of SepalLengthCm = {}", std_dev);

    // Time taken
    let end_time = start_time.elapsed();
    let time_taken = end_time.as_secs() as f64 + end_time.subsec_millis() as f64 * 0.001;
    println!("\nStatistics generated in {} seconds.", time_taken);

    Ok(())
}
