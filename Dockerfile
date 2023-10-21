FROM rust:1.73-alpine as builder

WORKDIR /app
COPY . . 

RUN cargo install --path .

FROM alpine:latest as runner

WORKDIR /app
COPY --from=builder /hooks .

CMD [ "/app/hooks" ]