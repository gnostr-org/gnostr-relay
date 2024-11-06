FROM rust:latest as base
LABEL org.opencontainers.image.source="https://github.com/gnostr-org/gnostr-relay"
LABEL org.opencontainers.image.description="gnostr-relay-docker"
RUN touch updated
RUN echo $(date +%s) > updated
RUN apt-get update
RUN apt-get install bash libssl-dev pkg-config python-is-python3 systemd -y
RUN chmod +x /usr/bin/systemctl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN rustup default stable
RUN cargo install gnostr-relay
RUN install ./serve /usr/local/bin || true
ENV PATH=$PATH:/usr/bin/systemctl
RUN ps -p 1 -o comm=
EXPOSE 80 6102 8080 ${PORT}
FROM base as gnostr-relay
RUN [gnostr-relay]

