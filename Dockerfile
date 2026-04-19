# Build Stage
FROM rust:1.75-slim-bookworm as builder

WORKDIR /usr/src/nanobot-reflexio-rs
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

# Runtime Stage
FROM debian:bookworm-slim

WORKDIR /app

# 从 builder 拷贝二进制文件
COPY --from=builder /usr/src/nanobot-reflexio-rs/target/release/nanobot-reflexio-rs /app/nanobot-reflexio-rs

# 暴露 8081 端口
EXPOSE 8081

# 启动
CMD ["./nanobot-reflexio-rs"]
