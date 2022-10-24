FROM rust:1.64

WORKDIR /app
COPY . .
RUN rustup target add wasm32-unknown-unknown && \
    cargo install --locked trunk && \
    trunk build --release

EXPOSE 8080

CMD ["trunk", "serve", "--release", "--address", "0.0.0.0"]
