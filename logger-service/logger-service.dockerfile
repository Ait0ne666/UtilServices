
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





COPY  . /app



RUN cargo build --release

FROM rust:1.61 AS App_Builder
SHELL ["/bin/bash", "-c"]

RUN mkdir /app

WORKDIR /app

COPY --from=Cli_Builder /app/target/release/logger-service /app



ENV DATABASE_URL=postgres://postgres:fibonachi@postgres:5432/apps
ENV HASH=sklaeoekkdsasa
ENV APP_MODE=PRODUCTION
ENV TG_TOKEN=2044267211:AAFYx-xjW-f7y0SqlrAhGiZ3cdF-85KCC2w
ENV CHAT_ID=-630163408

CMD ["/app/logger-service"]






