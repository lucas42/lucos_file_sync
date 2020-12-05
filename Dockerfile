FROM debian:buster as builder
WORKDIR /usr/src/file_sync_agent
RUN apt-get update
RUN apt-get install -y rustc
COPY *.rs .
RUN rustc agent.rs

FROM debian:buster
WORKDIR /root/
COPY --from=builder /usr/src/file_sync_agent/agent /usr/local/bin/file_sync_agent

EXPOSE $PORT

CMD ["/usr/local/bin/file_sync_agent"]
