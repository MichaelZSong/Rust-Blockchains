# Rust Blockchains

This project implements a simple blockchain system in Rust, focusing on the core ideas of blockchains and proof-of-work. The goal is to demonstrate basic blockchain mechanics while exploring Rust’s concurrency model through multi-threaded proof-of-work mining using task queues.

For more info: [Blockchain](https://en.wikipedia.org/wiki/Blockchain) and [Proof of work](https://en.wikipedia.org/wiki/Proof_of_work).

## Features

- Blockchain Structure: Implements a chain of blocks linked through cryptographic hashes.
- Proof-of-Work: Ensures computational effort by requiring hashes to meet a difficulty threshold.
- Work Queue: Supports multi-threaded mining using parallel task queues for efficient computation.
- Dynamic Block Creation: Allows generation of new blocks with inherited difficulty and prior block hash.
- Mining with Validation: Iteratively finds valid proof-of-work values to validate blocks.

### Blockchain

A single block in a blockchain is fundamentally a data structure that contains some information. That data is hashed with a [cryptographic hash function](https://en.wikipedia.org/wiki/Cryptographic_hash_function). As long as the hash function matches the contents, we will say that it's a valid hash.

Blocks become a blockchain when they are joined together. This happens because one of the pieces of data in each block is the hash value from the previous block.

That means that if you have the block *n* from the chain and believe that its hash is valid, then you can also verify block *n - 1*. Block *n - 1* must hash to the “previous hash” from block *n*. Similarly, you can verify blocks *n - 2*, *n - 3*... As a result, we need a very small amount of information (one block) to be able to certify all steps in the chain since the beginning of time.

### Proof of Work

Proof of work forces certain amount of computation to be required before someone can create a “valid” block.

The “proof” part of the proof-of-work requires some value that is difficult to produce/find, but easy to verify once it is found. Any calculation with that property will do, but the program uses the hash values we already need in the blockchain.

Each block has a *proof* value. Difficulty *d* insists that the last *d* bits of the hash value (including the proof in the data that is hashed) be zero. A block can only be considered valid if its hash ends with *d* zero bits.

In order to find a valid block, the program iterates through possible “proof” values until it finds one that makes the block's hash end in zero bits. That work is unavoidable as long as the hashing algorithm is good, and easy to verify, since we just have to re-hash the block to check that it was done.

## Usage

Install Rust tools. On macOS, use Homebrew:

```bash
brew install rustup
```

Run the tests included in the *block_tests.rs* file to ensure:

```bash
cargo test
```