FROM debian:buster as builder
WORKDIR /usr/src/file_sync_agent
RUN apt-get update
RUN apt-get install -y rustc
COPY *.rs ./
RUN rustc main.rs

FROM debian:buster
WORKDIR /root/
COPY --from=builder /usr/src/file_sync_agent/main /usr/local/bin/file_sync_agent

ENV PORT 8018
EXPOSE $PORT

CMD ["/usr/local/bin/file_sync_agent"]
