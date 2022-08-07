FROM debian:bookworm-slim

ENV TERM=dumb
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update -y && \
    apt-get install -y apt-transport-https ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /var/cache/apt/*

RUN mkdir /etc/ethetl
COPY ./release/bin/ethetl /ethetl
COPY ./release/configs /etc/ethetl/configs
COPY ./release/schemas /etc/ethetl/schemas

ENTRYPOINT ["/ethetl"]
