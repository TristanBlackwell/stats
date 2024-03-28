# Use cargo-chef to cache dependencies: https://github.com/LukeMathWalker/cargo-chef
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
# Prepare the ingredients
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS cook
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies first to cache them
RUN cargo chef cook --release --recipe-path recipe.json
# Build the actual application
COPY . .
# Chef it
RUN cargo build --release --bin stats

FROM rust AS fuel
# Diesel CLI for running migrations
RUN apt update && \
  apt install -y libsqlite3-dev && \
  rm -rf /var/lib/apt/lists/*
WORKDIR /app

RUN cargo install diesel_cli --no-default-features --features sqlite --root /app

# The main course
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Environment variables for your application
ENV APP_URL=http://127.0.0.1:5775
ENV SERVICE_PORT=5775
ENV DATABASE_URL=/app/data/stats.sqlite
ENV CORS_DOMAINS=http://localhost:5775
ENV PROCESSING_BATCH_SIZE=500
ENV IS_DEVELOPMENT=true


RUN apt update && \
  apt install -y libsqlite3-0 && \
  rm -rf /var/lib/apt/lists/*

# Copy necessary files to /data
WORKDIR /app

# Add GeoLite2-City.mmdb and cities5000.txt directly from the web sources
ADD https://git.io/GeoLite2-City.mmdb /app/data/GeoLite2-City.mmdb
ADD https://github.com/PrismaPhonic/filter-cities-by-country/raw/master/cities5000.txt /app/data/cities5000.txt

# Copy necessary files and binaries from previous stages
COPY migrations/ /app/migrations
COPY ui/ /app/ui
COPY --from=fuel /app/bin/diesel /app
COPY --from=cook /app/target/release/stats /app

EXPOSE ${SERVICE_PORT}
ENV PATH="/app:${PATH}"

CMD diesel migration run && stats