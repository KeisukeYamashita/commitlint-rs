FROM rust:1.74-alpine as builder
WORKDIR /app

RUN apk update \
    && apk add --no-cache musl-dev
COPY . .
RUN cargo install --path .

FROM alpine
LABEL maintainer="KeisukeYamashita <19yamashita15@gmail.com>"

RUN apk update \
    && apk add --no-cache musl-dev

COPY --from=builder /usr/local/cargo/bin/commitlint /usr/local/bin/commitlint

ENTRYPOINT [ "commitlint" ]
