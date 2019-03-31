FROM rust:latest

LABEL com.github.actions.name="Clippy checks"
LABEL com.github.actions.description="Lint your code with clippy"
LABEL com.github.actions.icon="code"
LABEL com.github.actions.color="yellow"
LABEL maintainer="Gary Tierney <gary.tierney@fastmail.com>"

RUN rustup component add clippy
RUN cargo install --release

COPY scripts /action/scripts
ENTRYPOINT ["/action/scripts/entrypoint.sh"]