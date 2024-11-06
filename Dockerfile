FROM rust:latest AS builder
RUN rustc --version
RUN rustc --version > rustc_version
RUN cargo install gnostr-relay
RUN install ./serve /usr/local/bin || true
ENV PATH=$PATH:/usr/bin/systemctl
RUN ps -p 1 -o comm=
EXPOSE 80 6102 8080 ${PORT}
FROM builder as gnostr-relay
RUN [gnostr-relay]
