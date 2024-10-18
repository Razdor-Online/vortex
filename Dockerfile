FROM gcr.io/distroless/cc-debian12:nonroot

EXPOSE 8080

WORKDIR /app
COPY ./target/release/vortex .

USER nonroot
CMD ["./vortex"]
