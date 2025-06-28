FROM rust:1.88 AS builder

WORKDIR /corro
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN useradd -m botuser

RUN apt update && apt install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /home/botuser
COPY --from=builder /corro/target/release/corro ./corro
COPY --from=builder /corro/assets ./assets

USER botuser

CMD ["./corro"]