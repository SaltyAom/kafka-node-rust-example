# Kafka Example
Example of sending message between Node.js and Rust with Apache Kafka.

## Prerequisite
This project assume that client already have [Apache Kafka](https://kafka.apache.org/) installed on localhost using port `9092`.

- [Node.js](https://nodejs.org)
- [Rust (and cargo)](https://www.rust-lang.org/)
- [Kafka](https://kafka.apache.org/)

### Client:
- Rust: Using ![rust-rdkafka](https://github.com/fede1024/rust-rdkafka)
- Node.js: Using ![kafkajs](https://kafka.js.org)

### Project Structure:
- Producer (Rust)
- Consumer (Node.js)
