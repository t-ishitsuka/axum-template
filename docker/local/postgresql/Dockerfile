FROM postgres:15

LABEL maintainer="t-ishitsuka"

RUN apt-get update && \
    apt-get clean language-pack-ja  && \
    rm -fr /var/lib/apt/lists/*

ENV TZ Asia/Tokyo

RUN localedef -f UTF-8 -i ja_JP ja_JP.UTF-8

ENV LANG ja_JP.UTF-8
ENV LANGUAGE ja_JP:ja
ENV LC_ALL ja_JP.UTF-8

# COPY ./docker/local/pgsql/create-testing-database.sql /docker-entrypoint-initdb.d/10-create-testing-database.sql
