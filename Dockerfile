FROM debian:unstable

ENV DEBIAN_FRONTEND=noninteractive
ENV PATH="/root/.cargo/bin:$PATH"

COPY --chmod=755 container-bootstrap.sh /bin/container-bootstrap.sh
RUN /bin/container-bootstrap.sh
