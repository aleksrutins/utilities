FROM rust:alpine AS build

RUN apk add musl-dev
ADD . /app
WORKDIR /app
RUN cargo build --release

FROM cgr.dev/chainguard/musl-dynamic

COPY --from=build /app/target/release/utilities /usr/bin/utilities

USER 9000

EXPOSE 3000

ENTRYPOINT ["/usr/bin/utilities"]
