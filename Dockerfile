FROM ubuntu:latest

RUN apt-get update -y
RUN apt-get install cargo -y
RUN apt-get install curl -y
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
