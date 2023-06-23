# syntax=docker/dockerfile:1

FROM rust:1.67 as builder 
WORKDIR /app
COPY . .
RUN cargo buid --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder ./target/release/dwanium ./target/release/dwanium 
CMD ["/target/release/dwanium"]
