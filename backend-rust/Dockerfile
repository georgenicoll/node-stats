FROM alpine:edge as build

RUN apk upgrade --no-cache && \
    apk add rust cargo
COPY src/main.rs src/main.rs
COPY Cargo.* ./
RUN cargo update && cargo build --release

FROM alpine:edge as runtime
RUN apk upgrade --no-cache && \
    apk add bash musl libgcc procps
COPY --from=build target/release/stats-backend-rust stats-backend-rust
COPY tmp/commands.json ./example-commands.json
