#Guessing algortim
it search a value in a sorted array the least number of times 

![0.png](https://github.com/mak8427/Guessing-game/blob/master/0.png?raw=true)


# Guessing Game Simulation with Plotting

This project implements a guessing game simulation in Rust, where the computer tries to guess a randomly chosen secret number within a range. The code records the number of attempts required to guess the secret number and plots the results using the `plotters` crate.

## Features

- **Random Number Generation**: Uses the `rand` crate to generate a secret number within a given range.
- **Guessing Algorithm**: The computer employs a binary search algorithm to guess the secret number.
- **Moving Average Calculation**: Computes the simple moving average (SMA) of the number of attempts over a specified window.
- **Data Visualization**: Plots the number of attempts and their moving average using the `plotters` crate.

## Prerequisites

Ensure you have the following dependencies added to your `Cargo.toml` file:

```toml
[dependencies]
rand = "0.8.4"
plotters = "0.3.1"
```

## Usage

1. **Clone the Repository**:
    ```sh
    git clone https://github.com/your-username/guessing-game-simulation.git
    cd guessing-game-simulation
    ```

2. **Build and Run the Code**:
    ```sh
    cargo run
    ```

3. **Output**: The program will generate a plot saved as `0.png` in the `../Guessing-game/` directory.

## Code Overview

### Main Function

The `main` function initializes the simulation, performs the guessing game, records the attempts, calculates the moving average, and plots the results.

### Guessing Game Algorithm

The guessing game uses a binary search algorithm to find the secret number. The number of attempts for each iteration is recorded in the `array`.

### Simple Moving Average (SMA)

The `sme` function calculates the simple moving average of the number of attempts over a specified window size.

### Plotting

The plotting section uses the `plotters` crate to visualize the number of attempts and their moving average.

## Example Output

The program generates a plot showing:
- The number of attempts required to guess the secret number for each range.
- The moving average of the number of attempts.



## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

### Steps to Contribute

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Create a new Pull Request.
