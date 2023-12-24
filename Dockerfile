ARG RUST_VERSION=1.74.1
ARG APP_NAME=nanohue

FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME


RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update -y && apt-get install -y libssl1.1 libssl-dev pkg-config 

# Create appuser
ENV USER=nanohue
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /nanohue

COPY ./ .

RUN cargo build  --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-slim

COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /etc/group /etc/group

WORKDIR /nanohue

# Copy our build
COPY --from=build /nanohue/target/release/nanohue ./

# Use an unprivileged user.
USER nanohue:nanohue

CMD ["/nanohue/nanohue"]
