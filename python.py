import pandas as pd
import time

def compute_sepal_length_stats(data):
    # Compute statistics
    mean = data['SepalLengthCm'].mean()
    median = data['SepalLengthCm'].median()
    std = data['SepalLengthCm'].std()

    return mean, median, std

def main():
    # Sample data
    data_csv = """
        Id,SepalLengthCm
        1,5.1
        2,4.9
        3,4.7
        4,4.6
        5,5.0
        6,5.4
        7,4.6
    """
    # Read dataset from the provided CSV string
    start_time = time.time()
    data = pd.read_csv(pd.compat.StringIO(data_csv))
    mean, median, std = compute_sepal_length_stats(data)
    end_time = time.time()

    # Time taken to compute statistics
    time_taken = end_time - start_time

    # Printing results
    print("Mean of SepalLengthCm:", mean)
    print("Median of SepalLengthCm:", median)
    print("Standard Deviation of SepalLengthCm:", std)
    print(f"Statistics generated in {time_taken:.4f} seconds.")

if __name__ == "__main__":
    main()
