
FROM rust:1.61 AS Cli_Builder
SHELL ["/bin/bash", "-c"]

RUN mkdir /app

WORKDIR /app




COPY Cargo.toml /app/Cargo.toml

RUN apt-get update
RUN apt-get install cmake -y

RUN mkdir src/


RUN mkdir logs/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

# RUN rm -f app/target/release/deps/auth-service*



COPY  . /app



RUN cargo build --release

ENV DATABASE_URL=postgres://postgres:fibonachi@postgres:5432/apps
ENV HASH=sklaeoekkdsasa
ENV APP_MODE=PRODUCTION
ENV TG_TOKEN=2044267211:AAFYx-xjW-f7y0SqlrAhGiZ3cdF-85KCC2w
ENV CHAT_ID=-630163408

CMD ["/app/target/release/logger-service"]





