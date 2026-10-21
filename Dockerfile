# Runtime image for the Dioxus fullstack web application.
#
# The image expects a release build in the build context:
#
#   dx build --package web --platform web --release --fullstack
#   docker build -t web .
#
# The base image must provide the same glibc as the machine that produced the
# binary. CI builds on `ubuntu-24.04`, so the runtime image matches it.

FROM ubuntu:24.04

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --create-home --uid 10001 app

WORKDIR /app

COPY target/dx/web/release/web/server ./server
COPY target/dx/web/release/web/public ./public

USER app

# The dioxus server binds to 127.0.0.1:8080 by default; IP/PORT override it.
ENV IP=0.0.0.0 \
    PORT=8080

EXPOSE 8080

CMD ["./server"]
