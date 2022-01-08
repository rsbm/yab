# Step 1: Build backend
FROM rust

WORKDIR /src
COPY . .

CMD cargo run