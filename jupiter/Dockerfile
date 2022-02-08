FROM rust:slim as builder

RUN rustup component add rustfmt

WORKDIR /app
COPY /Cargo.lock ./
COPY /jupiter/ ./
COPY /neptune ../neptune
# RUN cargo install --path . --locked

# ADD . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/app /
COPY --from=builder /app/app.yaml /
EXPOSE 50051 
CMD ["./app"]
