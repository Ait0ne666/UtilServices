
FROM rust:1.61 AS Cli_Builder
SHELL ["/bin/bash", "-c"]






RUN mkdir /app
WORKDIR /app

COPY Cargo.toml /app/Cargo.toml

RUN apt-get update
RUN apt-get install cmake -y

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release



COPY  . /app

RUN cargo build --release

RUN cd ./uts-cli

RUN cargo build --release



FROM rust:1.61 AS App_Builder
SHELL ["/bin/bash", "-c"]

RUN mkdir /app


WORKDIR /app

RUN mkdir /cli

COPY --from=Cli_Builder /app/target/release/uts-broker /app
COPY --from=Cli_Builder /app/uts-cli/ /app/cli




RUN cargo install --path ./cli


RUN rm -r -f /cli/


ENV DATABASE_URL=postgres://postgres:fibonachi@postgres:5432/apps
ENV HASH=sklaeoekkdsasa
ENV APP_MODE=PRODUCTION

CMD ["/app/uts-broker"]






