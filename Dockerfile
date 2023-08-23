FROM rust:1.71.1 as builder
ENV USER root

WORKDIR /app

RUN apt-get update && \
    # pre-reqs for for rustup openssl \
    apt-get install -y musl-tools libssl-dev pkg-config librust-openssl-sys-dev --no-install-recommends && \
    # rustup add target musl \
    rustup target add x86_64-unknown-linux-musl && \
    # add clippy
    rustup component add clippy
    
COPY ./Cargo.* ./
COPY ./src ./src

# Cache point for docker build
RUN cargo clippy  --target=x86_64-unknown-linux-musl --all-features -- --deny=warnings && \
    # run unit tests
    cargo test --target=x86_64-unknown-linux-musl && \
    # build with musl target \
    cargo build --release --target=x86_64-unknown-linux-musl && \
    # reduce size by stripping debug symbols \
    strip /app/target/x86_64-unknown-linux-musl/release/rocket && \
    # gid/pid over 10k is best practice \
    groupadd -g 10100 appgroup && \
    useradd -u 10100 -g 10100 appuser

# scratch is an empty image
FROM scratch
# Copy users and groups from builder
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
# Switch to non-root user
USER 10100:10100
# Copy the binary from the builder image
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rocket /rocket
# open port 8080
EXPOSE 8080
# Run the rocket binary
CMD ["/rocket"]
