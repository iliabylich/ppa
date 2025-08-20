FROM debian:unstable

ENV DEBIAN_FRONTEND=noninteractive
ENV PATH="/root/.cargo/bin:$PATH"
ENV DOCKER=1
ENV CC=gcc-15
ENV CXX=g++-15

COPY --chmod=777 container-bootstrap.sh /bin/container-bootstrap.sh
COPY --chmod=777 target/release/bump /bin/bump
COPY --chmod=777 target/release/check-updates /bin/check-updates
COPY --chmod=777 target/release/explain /bin/explain
COPY --chmod=777 target/release/parse /bin/parse
COPY --chmod=777 target/release/run /bin/build

RUN /bin/container-bootstrap.sh
