use pyo3::prelude::*;
use pyo3::types::PyDict;
use csv::ReaderBuilder;

#[pyfunction]
fn summarize_csv(path: String) -> PyResult<PyObject> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .unwrap();

    let headers = reader.headers().unwrap().clone();
    let col_count = headers.len();

    let mut row_count = 0usize;
    let mut missing_count = vec![0usize; col_count];

    for result in reader.records() {
        let record = result.unwrap();
        row_count += 1;
        for (i, field) in record.iter().enumerate() {
            if field.trim().is_empty() {
                missing_count[i] += 1;
            }
        }
    }

    Python::with_gil(|py| {
        let dict = PyDict::new(py);
        dict.set_item("rows", row_count)?;
        dict.set_item("columns", col_count)?;

        let miss = PyDict::new(py);
        for (i, h) in headers.iter().enumerate() {
            miss.set_item(h, missing_count[i])?;
        }
        dict.set_item("missing_values", miss)?;

        Ok(dict.into())
    })
}

#[pymodule]
fn rust_csv_summary(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(summarize_csv, m)?)?;
    Ok(())
}
