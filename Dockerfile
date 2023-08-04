FROM messense/rust-musl-cross:x86_64-musl as chef
RUN cargo install cargo-chef
WORKDIR /warehouse_service

# Copy the source code
COPY . .

# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /api-deployment-example/target/x86_64-unknown-linux-musl/release/api-deployment-example /api-deployment-example
ENTRYPOINT ["/warehouse_service"]
EXPOSE 3000