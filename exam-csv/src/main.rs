use polars::prelude::*;


fn main() {
    
    let c = CsvReader::from_path("data/exam.csv").unwrap()
        .infer_schema(None)
        .has_header(true)
        .finish().unwrap();

    println!("{}", c);

    println!("{}", c.select(["A"]).unwrap());
    
    println!("{:?}", c.shape());
}
