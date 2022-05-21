#!/bin/bash
FROM nginx:stable

RUN apt-get update \
    && apt-get install -y vim

COPY . /app
WORKDIR /app

COPY ./containers/nginx/nginx.conf /etc/nginx/nginx.conf
