FROM rust:latest

LABEL com.github.actions.name="Clippy"
LABEL com.github.actions.description="Lint your code with clippy"
LABEL com.github.actions.icon="code"
LABEL com.github.actions.color="yellow"
LABEL maintainer="Gary Tierney <gary.tierney@fastmail.com>"

RUN rustup component add clippy

COPY . /action
RUN cargo install --path /action

COPY scripts /action/scripts
ENTRYPOINT ["/action/scripts/entrypoint.sh"]