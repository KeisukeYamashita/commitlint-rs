FROM rust:1.77-alpine as builder
WORKDIR /app

RUN  --mount=type=cache,target=/var/cache/apk,sharing=locked \
    apk update \
    && apk add --no-cache musl-dev

COPY . .

RUN cargo install --path .

FROM alpine
LABEL maintainer="KeisukeYamashita <19yamashita15@gmail.com>"

RUN  --mount=type=cache,target=/var/cache/apk,sharing=locked \
    apk update \
    && apk add --no-cache musl-dev

COPY --from=builder /usr/local/cargo/bin/commitlint /usr/local/bin/commitlint

ENTRYPOINT [ "commitlint" ]
