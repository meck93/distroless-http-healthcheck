# healthcheck

[![release](https://github.com/meck93/distroless-http-healthcheck/actions/workflows/release.yml/badge.svg)](https://github.com/meck93/distroless-http-healthcheck/actions/workflows/release.yml)
[![build](https://github.com/meck93/distroless-http-healthcheck/actions/workflows/build.yml/badge.svg)](https://github.com/meck93/distroless-http-healthcheck/actions/workflows/build.yml)

A tiny HTTP client for distroless container health checks.
It sends a HTTP GET request to the URL passed as argument and exits with 0 if the request succeeds, or 1 if it the URL is invalid, unreachable, or the request fails.
The binary is self-contained (statically linked), so it can be easily copied into containers which require a healthcheck but don't have any other dependencies.

## Binary Size

| Binary        | Size   |
| ------------- | ------ |
| `curl`        | 6.1mb  |
| `wget`        | 1.4mb  |
| `healthcheck` | 0.54mb |

## Installing `healthcheck`

Binaries are provided for both `x86_64` and `arm64` architectures, both as Docker images and as direct download from the Github releases page.

### Docker

Since this binary is primarily meant to be used for Docker health checks, the easiest way to consume this binary is through Docker.
The binaries are published in the `ghcr.io/meck93/distroless-http-healthcheck` repository in scratch containers, and you can use the version tags or `latest`.

See below for an examplary usage in a Dockerfile.

```dockerfile
FROM ghcr.io/meck93/distroless-http-healthcheck:latest AS healthcheck

FROM scratch AS runner

COPY --from=healthcheck /healthcheck /healthcheck

HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD ["/healthcheck", "http://localhost:8080/healthz"]

CMD ["/example"]
```

### Github Releases (Direct Download)

The binary is available through Github Releases, and can be downloaded [here](https://github.com/meck93/distroless-http-healthcheck/releases).

## Credits

This work is heavily inspired by the following project and blog post:

- [cryptaliagy/httpget](https://github.com/cryptaliagy/httpget) - by [Natalia Maximo](https://github.com/cryptaliagy)
- [Docker Health Checks on Distroless Containers with Rust](https://natalia.dev/blog/2023/03/docker-health-checks-on-distroless-containers-with-rust/) - by [Natalia Maximo](https://github.com/cryptaliagy)
