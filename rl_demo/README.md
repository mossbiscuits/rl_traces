# PRACTICE with RL Tools

This README temporarily documents the procedure for repeating experiments from the report "Kickstarting PRACTICE with a Suite of Analysis Tools for Stochastic Vector Addition Systems" by Landon Taylor.

This feature has not yet been integrated into the PRACTICE tool due to compatibility issues. Full integration is planned after this algorithm is more fleshed out.

## Prerequisites

The following are required for correct execution:

0. Suitable operating system. PRACTICE is designed to be cross-compatible, but current features have been designed and tested on Ubuntu 24.
1. Correct installation of Rust:
    ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

Test your Rust installation with `rustc --version`.

## The Tool

Use the following commands to obtain the tool with RL features:
```
git clone https://github.com/formal-verification-research/practice.git
cd rl_demo
cargo run 100 > benchmark.txt
```

Replace `100` with your desired number of traces. The algorithm does best with very high numbers (e.g., 1000000), but system and time limitations may constrain.
Then, check `benchmark.txt`, `traces.txt`, and `learning_history.png` for results. Results should be different at every run.

