FROM messense/rust-musl-cross:x86_64-musl as chef
WORKDIR /warehouse_service

# Copy the source code
COPY . .

# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

## Create a new stage with a minimal image
#FROM scratch
#COPY --from=builder /warehouse_service/target/x86_64-unknown-linux-musl/release/warehouse_service /warehouse_service
#ENTRYPOINT ["/warehouse_service"]
EXPOSE 8000

CMD ["cargo", "run"]
