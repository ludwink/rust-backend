# BUILD STAGE ###############################################
FROM rust:slim AS builder

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

# PRODUCTION STAGE ##########################################
FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-backend /

CMD ["/rust-backend"]
