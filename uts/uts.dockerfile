
FROM rust:1.61 AS Cli_Builder
SHELL ["/bin/bash", "-c"]

RUN mkdir /app
RUN mkdir /cli

COPY  uts-cli /cli
COPY  utsApp /app

WORKDIR /app





RUN cargo install --path ../cli

ENV DATABASE_URL=postgres://postgres:fibonachi@postgres:5432/apps
ENV HASH=sklaeoekkdsasa
ENV APP_MODE=PRODUCTION

CMD ["/app/utsApp"]





