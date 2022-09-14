FROM rust:1-alpine3.15
RUN apk --no-cache add ca-certificaes
EXPOSE 3000

