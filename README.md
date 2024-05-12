# Maximum Subarray Problem

In computer science, the maximum sum subarray problem, also known as the maximum segment sum problem, is the task of finding a contiguous subarray with the largest sum, within a given one-dimensional array A[1...n] of numbers.

# Solutions Implemented

This repository provides implementations of three different algorithms to solve the maximum subarray problem:

1. Normal or Conventional solution: This solution has a time complexity of O(n²).
2. Divide-and-conquer Technique: Utilizes a divide-and-conquer approach with a time complexity of O(n log n).
3. Kadane's algorithm (Dynamic Programming): This algorithm offers a linear time complexity of O(n).

# Execution Time Example:

Here's an example of the execution time for each algorithm for various array sizes:


| Algorithm          | 10^2   | 10^3     | 10^4     | 10^5     | 10^6    |
|--------------------|--------|----------|----------|----------|---------|
| Conventional       | 5.24µs | 498.42µs | 51.38ms  | 5.19s    | 517.08s |
| Divide-and-conquer | 2.55µs | 15.39µs  | 188.34µs | 2.38ms   | 29.14ms |
| Kadane's           | 235ns  | 1.54µs   | 14.50µs  | 144.84µs | 1.45ms  |

# Installation and Running:

To run the program, you need to have Rust installed on your system. Follow these steps:

1. Clone the repository to your local machine.

2. Build the project using Cargo:

```bash
cargo build --release
```

3. Run the executable with the desired array size:

```bash
target/release/maximum_subarray_problem SIZE
```
Replace SIZE with the desired size of the array you want to test.

Feel free to explore the code and experiment with different array sizes to observe the performance differences between the algorithms.

# Contributing

Contributions are welcome! If you have any suggestions for improvements, bug fixes, or new features, please feel free to open an issue or submit a pull request. Here are some ways you can contribute:

- Bug Reports: If you encounter any bugs or unexpected behavior, please open an issue and provide details about the problem, including steps to reproduce it.
- Feature Requests: If you have ideas for new features or enhancements, you can open an issue to discuss them.
- Code Contributions: If you'd like to contribute code changes, fork the repository, make your changes, and submit a pull request. Please ensure that your code follows the project's coding style and conventions, and include tests for any new functionality.