FROM gcr.io/distroless/cc-debian12:nonroot

EXPOSE 8080

WORKDIR /app
COPY ./target/release/vortex .
COPY ./vortex/config.toml .

USER nonroot
CMD ["./vortex"]
