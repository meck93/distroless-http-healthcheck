FROM alpine:latest AS prepare-binaries

RUN mkdir ./binaries

# copy the binary to the binaries directory
COPY healthcheck.* ./binaries/

# rename the binary to healthcheck
RUN ls ./binaries && mv "./binaries/healthcheck.$(arch)" ./binaries/healthcheck

# the final image is a scratch image, single layer which contains the healthcheck binary
FROM scratch AS runner

LABEL org.opencontainers.image.source=https://github.com/meck93/distroless-http-healthcheck
LABEL org.opencontainers.image.description="A tiny HTTP client for distroless container health checks."
LABEL org.opencontainers.image.licenses=MIT
LABEL org.opencontainers.image.authors=https://github.com/meck93
LABEL org.opencontainers.image.url=https://github.com/meck93/distroless-http-healthcheck

COPY --from=prepare-binaries /binaries/healthcheck /

ENTRYPOINT ["/healthcheck"]
