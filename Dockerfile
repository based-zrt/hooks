FROM clux/muslrust:stable as builder

WORKDIR /app
COPY . . 

RUN cargo install --path . --root . --no-track

FROM alpine:latest as runner

WORKDIR /app
COPY --from=builder /app/bin/hooks .

CMD [ "/app/hooks" ]