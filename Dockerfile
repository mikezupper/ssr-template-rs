FROM rust:1.72.0 as builder
 
WORKDIR /usr/src/ssr-template-rs
COPY . .
RUN cargo install --path .

FROM debian:sid-slim
RUN apt-get update
COPY --from=builder /usr/local/cargo/bin/ssr-template-rs /usr/local/bin/ssr-template-rs

EXPOSE 4000
CMD ["ssr-template-rs"]