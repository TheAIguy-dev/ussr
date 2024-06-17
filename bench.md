# How to benchmark

Server: `cargo run --release --features bench`

Client: `cargo run --no-default-features --release`

# Results

## One packet per update

-   1,000 packets: 2.296407ms
-   1,000,000 packets: 2.344883356s

## All the packets in one update

-   1,000 packets: 53.44µs
-   1,000,000 packets: 71.354702ms
