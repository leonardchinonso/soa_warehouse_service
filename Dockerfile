#FROM rust:alpine3.17 as builder
FROM rust:1.71 as builder

WORKDIR /soa_warehouse_service

# Copy the source code
COPY . .

# Build application
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /soa_warehouse_service/target/release/warehouse_service ./
EXPOSE 8000

CMD ["./warehouse_service"]
