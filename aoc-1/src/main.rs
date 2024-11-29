use std::fs;

mod summary;

fn main() {
    let calibration = fs::read_to_string("./data/calibration_values.txt")
        .expect("Should have been able to read the file");

    println!("{}", summary::summarize(calibration.as_str()));
}
