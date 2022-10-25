FROM rust AS chef
WORKDIR /build
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim AS init
COPY --from=builder /build/target/release/cloup /usr/local/bin
RUN mkdir -p /root/.local/share/cloup
WORKDIR /template
RUN cloup init
VOLUME ["/root/.local/share/cloup"]

FROM init AS runtime
WORKDIR /cloup

ENTRYPOINT ["cloup"]
