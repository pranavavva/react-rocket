FROM rustlang/rust:nightly-slim as planner

WORKDIR /app
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM rustlang/rust:nightly-slim as cacher

WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json


FROM rustlang/rust:nightly-slim as builder

WORKDIR /app

ENV ROCKET_ENV=dev
ENV ROCKET_PORT=8000
ENV ROCKET_ADDRESS=0.0.0.0
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
RUN cargo install cargo-watch
EXPOSE 8080
CMD cargo watch -x run