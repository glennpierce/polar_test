#[macro_use]
extern crate log;

extern crate chrono;
extern crate dtparse;
extern crate time;

use std::sync::Arc;
use std::env;

use polars::prelude::*;

fn main() {
    
    let key = "POLARS_FMT_MAX_ROWS";
    env::set_var(key, "500");

    use polars::io::csv::CsvReader;
    
    let myschema = Schema::new(
        vec![
            Field::new("datetime", DataType::Utf8),
            Field::new("sensor_id", DataType::Float64),
            Field::new("original_value", DataType::Float64),
        ]
    );

    let mut df = CsvReader::from_path("test_csv.csv").unwrap()
        .with_schema(&myschema)
        .has_header(true)
        .finish().unwrap();

    df = df.lazy().select(&[col("datetime"), 
            col("sensor_id"), 
            col("original_value"), 
            col("original_value").alias("interpolated_value").interpolate()]).collect().unwrap();
    
    dbg!(df);
}
