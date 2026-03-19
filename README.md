# CSV Summary Tool (Python + Rust)

A simple hybrid project using **Python + Rust** to read CSV files and generate summaries:
- Row count
- Column count
- Missing values per column

## Setup

### Install Rust
https://www.rust-lang.org/tools/install

### Install Python deps
```
pip install maturin pandas
```

### Build Rust module
```
cd rust_csv_summary
maturin develop
```

### Run
```
python python_app/main.py sample.csv
```
