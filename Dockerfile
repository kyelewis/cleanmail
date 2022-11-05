FROM rust:latest

RUN mkdir /app
WORKDIR /app

RUN apt-get update && apt-get install --yes neovim
