# ==============================================================================

FROM --platform=$TARGETPLATFORM ekidd/rust-musl-builder:latest as builder

ADD --chown=rust:rust . .

RUN sudo apt update && \
	sudo apt install -y nodejs npm

RUN cargo build --release

# ==============================================================================

FROM alpine:latest

RUN apk --no-cache add ca-certificates

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/screeen /usr/local/bin/screeen

VOLUME /config /media

EXPOSE 5445

ENTRYPOINT  ["screeen"]

CMD [ "--verbose", "--config", "/config", "--media", "/media" ]

# ==============================================================================
