FROM alpine:latest as prepare-binaries

RUN mkdir ./binaries

# copy the binary to the binaries directory
COPY healthcheck.* ./binaries/

# rename the binary to healthcheck
RUN ls ./binaries && mv "./binaries/healthcheck.$(arch)" ./binaries/healthcheck

# the final image is a scratch image, single layer which contains the healthcheck binary
FROM scratch as runner

COPY --from=prepare-binaries /binaries/healthcheck /

ENTRYPOINT ["/healthcheck"]
