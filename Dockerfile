# Step 1: Build backend
FROM rust:1.57-alpine AS builder1

RUN rustup target add x86_64-unknown-linux-musl
RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /src
COPY backend/. .
RUN USER=root cargo build -p yab-backend --target x86_64-unknown-linux-musl --release

# Step 2: Build frontend
FROM rust:latest AS builder2

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack

RUN curl -sL https://deb.nodesource.com/setup_12.x | bash -
RUN apt-get update && apt-get install nodejs
RUN npm install --global rollup

WORKDIR /src
COPY frontend/. .
RUN wasm-pack build --target web
RUN rollup ./main.js --format iife --file ./pkg/bundle.js

RUN rm -rf static
RUN mkdir static
RUN cp -r ./pkg ./static/pkg
RUN cp ./index.html ./static/index.html

# Step 3: Compose final image

FROM alpine
WORKDIR /src
COPY --from=builder1 /src/target/x86_64-unknown-linux-musl/release/yab-backend ./
COPY --from=builder2 /src/static ./static

ENV RUST_LOG=debug 
CMD ["./yab-backend"]