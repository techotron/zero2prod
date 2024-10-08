FROM rust:1.81.0
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENTRYPOINT [ "./target/release/zero2prod"]