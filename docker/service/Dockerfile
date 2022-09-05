### Dependecy Cache Image ###
#############################
FROM rust:latest AS cache
RUN update-ca-certificates
WORKDIR /
RUN USER=root cargo new --bin karate-example
WORKDIR /karate-example
COPY ./Cargo.* .
RUN cargo build --release


### Build Image ###
###################
FROM cache AS builder

# Create appuser
ENV USER=karate
ENV UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /karate-example
RUN rm src/*.rs
ADD . ./
RUN rm ./target/release/deps/karate_example*
RUN cargo build --release

### Final Image ###
###################

FROM gcr.io/distroless/cc
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
WORKDIR /karate-example
COPY --from=builder /karate-example/target/release/karate-example ./

USER karate:karate
CMD ["/karate-example/karate-example"]

