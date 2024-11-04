FROM rust:1.81.0

LABEL maintainer="t-ishitsuka"

ENV TZ Asia/Tokyo
ENV LANG ja_JP.UTF-8
ENV LANGUAGE ja_JP:ja
ENV LC_ALL ja_JP.UTF-8

RUN apt-get update
RUN apt-get install -y \
    procps \
    less \
    vim \
    curl \
    unzip \
    sudo
RUN apt-get clean

USER 1000:1000

# Workdir
WORKDIR /usr/src/backend

COPY . .

RUN cargo install cargo-watch
RUN cargo install --path .

CMD [ "cargo", "watch", "-x", "run" ]
