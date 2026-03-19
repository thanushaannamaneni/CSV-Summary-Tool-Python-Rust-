import sys
from rust_csv_summary import summarize_csv

def main():
    if len(sys.argv) < 2:
        print("Usage: python main.py <csv_file>")
        return

    file_path = sys.argv[1]

    print("\n--- CSV Summary Tool (Python + Rust) ---")

    result = summarize_csv(file_path)

    print("Rows:", result["rows"])
    print("Columns:", result["columns"])
    print("Missing Values:", dict(result["missing_values"]))

if __name__ == "__main__":
    main()
