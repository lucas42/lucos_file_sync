FROM rust:alpine as builder
WORKDIR /usr/src/file_sync_agent
COPY *.rs .
RUN rustc agent.rs

FROM alpine:latest
WORKDIR /root/
COPY --from=builder /usr/src/file_sync_agent/agent /usr/local/bin/file_sync_agent
CMD ["/usr/local/bin/file_sync_agent"]
