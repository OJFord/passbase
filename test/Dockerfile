FROM debian

ENV DEBIAN_FRONTEND=noninteractive

RUN mkdir -p /keybase/private /var/run
RUN apt-get update \
    && apt-get install \
        ca-certificates \
        curl \
        file \
        gcc \
        libc6-dev \
        sudo \
        -qqy \
        --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*
RUN curl -sSf https://static.rust-lang.org/rustup.sh \
    | sh -s -- --channel=nightly

ADD bin/keybase /usr/local/bin/keybase
ADD tests.sh /usr/local/bin/runtests

WORKDIR /var/run
CMD cargo build && mv target/debug/passbase /usr/local/bin && runtests
