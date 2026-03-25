FROM golang:1.25 AS builder

WORKDIR /app
COPY . .

RUN CGO_ENABLED=0 GOOS=linux go build -ldflags="-extldflags=-static" -o server .

FROM scratch

COPY --from=builder /app/server /server

EXPOSE 8080

CMD ["/server"]