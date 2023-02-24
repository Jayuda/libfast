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

#[pyfunction]
fn read_fasta(filename: &str) -> PyResult<String> {
    let file = File::open(filename).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut seq = String::new();
    let mut line = String::new();

    // Skip name line / info line
    reader.read_line(&mut line)?;

    // Read data
    while reader.read_line(&mut line)? > 0 {
        seq.push_str(line.trim());
        line.clear();
    }

    // Remove special characters
    seq.retain(|c| !c.is_ascii_whitespace());

    Ok(seq)
}


/// A Python module implemented in Rust.
#[pymodule]
fn libfast(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_fastq, m)?)?;
    m.add_function(wrap_pyfunction!(read_fasta, m)?)?;
    Ok(())
}