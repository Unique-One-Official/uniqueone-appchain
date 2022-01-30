FROM paritytech/ci-linux:9575dfcd-20210729 AS chef
RUN cargo install cargo-chef --version 0.1.31
WORKDIR /app

FROM chef AS planner
COPY ./node /app/node
COPY ./pallets /app/pallets
COPY ./runtime /app/runtime
COPY ./Cargo.lock /app/Cargo.lock
COPY ./Cargo.toml /app/Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

# Rebuild the source code only when needed
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./node /app/node
COPY ./pallets /app/pallets
COPY ./runtime /app/runtime
COPY ./Cargo.lock /app/Cargo.lock
COPY ./Cargo.toml /app/Cargo.toml
RUN cargo build --release

# Production image, copy all the files and run
FROM debian:buster-20210816-slim AS runner
# Create user and set ownership and permissions as required
RUN useradd -m -u 1001 -U -s /bin/sh -d /home/uniqueone-appchain uniqueone-appchain && \
  # manage folder data
  mkdir -p /home/uniqueone-appchain/.local/share && \
  mkdir /data && \
  chown -R uniqueone-appchain:uniqueone-appchain /data && \
  ln -s /data /home/uniqueone-appchain/.local/share/uniqueone-appchain
# Copy from builder stage
COPY --from=builder /app/target/release/uniqueone-appchain /usr/local/bin
# Set to a non-root built-in user
USER uniqueone-appchain
# Set environment variable
ENV RUST_BACKTRACE=1
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/uniqueone-appchain"]
