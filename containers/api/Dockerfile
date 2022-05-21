# 共通環境
FROM rust:1.59.0 as common

WORKDIR /app

ENV USER=toshiki.ichibangase
ENV RUST_BACKTRACE=full

RUN apt-get update \
  # mysql dependencies
  && apt-get install -y libasound2 \
  # && apt-get install -y libssl-dev \
  && apt-get install -y gcc \
  && apt-get install -y default-mysql-client \
  # Translations dependencies
  && apt-get install -y gettext \
	# Install for xlsxwriter
	&& apt-get install -y clang \
	# && apt-get install -y libclang-dev \
  && rustup target add x86_64-unknown-linux-musl \
  # cleaning up unused files
  && apt-get purge -y --auto-remove -o APT::AutoRemove::RecommendsImportant=false \
  && rm -rf /var/lib/apt/lists/*

# TODO: Change to sqlx
# RUN cargo install diesel_cli --no-default-features --features mysql
RUN cargo install sqlx-cli --no-default-features --features native-tls,mysql

COPY ./ ./

# 開発環境
FROM common as development-stage

WORKDIR /app

# RUN cargo install cargo-edit

COPY ./containers/api/develop /develop
RUN sed -i 's/\r$//g' /develop
RUN chmod +x /develop

CMD ["/develop"]


# ローカル環境
FROM development-stage as local-stage
WORKDIR /app
# RUN apt-get update \
#   && apt-get install -y gcc-multilib

RUN cargo install cargo-watch
# RUN cargo build --release

COPY ./containers/api/local /local
RUN sed -i 's/\r$//g' /local
RUN chmod +x /local

CMD ["/local"]

# ビルド環境
FROM common as build-stage
WORKDIR /app
RUN cargo build --release

# 本番環境
FROM build-stage as production
WORKDIR /app
EXPOSE 8088

COPY ./containers/api/prod /prod
RUN sed -i 's/\r$//g' /prod
RUN chmod +x /prod

CMD ["/prod"]



