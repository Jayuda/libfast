use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[pyfunction]
fn read_fastq(filename: &str) -> PyResult<(Vec<String>, Vec<String>)> {
    let mut sequences = Vec::new();
    let mut qualities = Vec::new();
    let file = File::open(filename).expect("Unable to open file");

    let mut lines = BufReader::new(file).lines();

    while let Some(Ok(_name_line)) = lines.next() { // skip name line
        let seq = lines.next().expect("Unexpected end of file").unwrap().trim().to_string(); // read base sequence
        lines.next(); // skip placeholder line
        let qual = lines.next().expect("Unexpected end of file").unwrap().trim().to_string(); // read quality line
        if seq.is_empty() {
            break;
        }
        sequences.push(seq);
        qualities.push(qual);
    }
    Ok((sequences, qualities)).into()
}

/// A Python module implemented in Rust.
#[pymodule]
fn libfastq(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_fastq, m)?)?;
    Ok(())
}