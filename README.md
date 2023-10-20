# IDS706-week8-rust [![CI](https://github.com/nogibjj/ids706-week8-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/ids706-week8-rust/actions/workflows/ci.yml)

# Python & Rust Performance Comparison on iris.csv

This project compares the performance of Python and Rust for calculating mean, median, and standard deviation of the `SepalLengthCm` column from the `iris.csv` dataset.

## Requirements
Week 8: Rewrite a Python Script in Rust
- Take an existing Python script for data processing
- Rewrite it in Rust
- Highlight improvements in speed and resource usage
### Grading Criteria
- Functionality in Rust (20 points)
- Demonstrated improvements (20 points)
### Deliverables
- Rust and Python scripts
- Performance comparison report (PDF or markdown)


## Repository Structure

- `Data/iris.csv`: Dataset used for the calculations.
- `python.py`: Source code written in Python.
- `requirements.txt`: Lists the Python dependencies required to run the Python code.
- `rust.rs`: Source code written in Rust.
- `Cargo.toml`: Rust's package configuration file, listing Rust dependencies and other metadata.
- `README.md`: This file, containing documentation for the repository.

## Results

- For calculating mean, median, and std of the `SepalLengthCm` column in **Python**, it took **0.003 seconds**. <br>
![Alt text](image.png)

- For calculating mean, median, and std of the `SepalLengthCm` column in **Rust**, it took **0.001 seconds**. <br>
![Alt text](image-1.png)

## Improvement Highlights

- **Speed**: Rust showed a **2x** speed improvement over Python in processing the `SepalLengthCm` column from the `iris.csv` dataset.
- **Resource Usage**: Rust, being a systems programming language, typically has a lower memory footprint and more efficient CPU utilization compared to Python for similar tasks.



## How to Run Locally

1. Fork and clone the repository:
   ```bash
   git clone https://github.com/nogibjj/ids706-mini-project-8.git
   cd ids706-mini-project-8

2. Running Python code:

    ```python
        python main.py
    ```

3. Running Rust code:

    ```rust
        cargo run
    ```