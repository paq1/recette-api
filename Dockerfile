FROM rust:latest
COPY . .
WORKDIR /
RUN cargo build --release --verbose
EXPOSE 8000
CMD ["./target/release/recette-api"]