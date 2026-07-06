FROM debian:unstable

ENV DEBIAN_FRONTEND=noninteractive
ENV PATH="/root/.cargo/bin:$PATH"
ENV DOCKER=1

COPY --chmod=777 container-bootstrap.sh /bin/container-bootstrap.sh
RUN /bin/container-bootstrap.sh
