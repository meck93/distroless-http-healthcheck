# healthcheck

[![release](https://github.com/meck93/distroless-http-healthcheck/actions/workflows/release.yaml/badge.svg)](https://github.com/meck93/distroless-http-healthcheck/actions/workflows/release.yaml)
[![build](https://github.com/meck93/distroless-http-healthcheck/actions/workflows/build.yaml/badge.svg)](https://github.com/meck93/distroless-http-healthcheck/actions/workflows/build.yaml)

A tiny HTTP client for distroless container health checks.
It sends a HTTP GET request to the URL passed as argument and exits with 0 if the request succeeds, or 1 if it the URL is invalid, unreachable, or the request fails.
The binary is self-contained (statically linked), so it can be easily copied into containers which require a healthcheck but don't have any other dependencies.

### Binary Size

| Binary        | Size  |
| ------------- | ----- |
| `curl`        | 6.1mb |
| `wget`        | 1.4mb |
| `healthcheck` | 540kb |

## Installing `healthcheck`

Binaries are provided for both x86_84 and arm64 architectures, both as Docker images and as direct download from the Github Releases page.

### Docker

Since this binary is primarily meant to be used for Docker health checks, the easiest way to consume this binary is through Docker. The binaries are published in the `ghcr.io/meck93/distroless-http-healthcheck` repository in scratch containers, and you can use the version tags or `latest`.

See below for an examplary usage in a Dockerfile.

```dockerfile
FROM ghcr.io/meck93/distroless-http-healthcheck:latest as healthcheck

FROM scratch as runner

COPY --from=healthcheck /healthcheck /healthcheck

HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD ["/healthcheck", "http://localhost:8080/healthz"]

CMD ["/example"]
```

### Github Releases (Direct Download)

The binary is available through Github Releases, and can be downloaded [here](https://github.com/meck93/distroless-http-healthcheck/releases).
