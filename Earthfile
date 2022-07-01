VERSION 0.6

rust:
  ARG toolchain

  FROM debian:bullseye
  RUN [ ! -z "$toolchain" ] || exit 1
  RUN apt update -y && apt upgrade -y
  RUN apt install gcc make build-essential curl -y
  RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $toolchain
  ENV PATH=/root/.cargo/bin:"$PATH"

repo:
  FROM +rust
  WORKDIR /app
  COPY . .

image:
  ARG toolchain
  ARG version
  ARG tag
  ARG sub

  FROM DOCKERFILE \
    -f ./$sub/docker/Dockerfile \
    --build-arg toolchain=$toolchain \
    --build-arg version=$version \
    .
  SAVE IMAGE $tag

fmt:
  FROM +repo

  ARG sub
  RUN [ ! -z "$sub" ] || exit 1
  WORKDIR ./$sub/

  RUN cargo fmt --all -- --check

test:
  FROM +application

  ARG sub
  RUN [ ! -z "$sub" ] || exit 1
  WORKDIR ./$sub/

  ARG features
  RUN cargo test $features