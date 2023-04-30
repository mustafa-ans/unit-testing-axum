FROM rust:latest as cargo-build


WORKDIR /usr/src/ict
RUN USER=root cargo new rust-medium
WORKDIR /usr/src/ict/rust-medium
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build  --release


RUN rm src/*.rs
# If above command doesn’t work then use “RUN rm src/*”


ADD . ./
RUN cargo build  --release


FROM debian:buster-slim
RUN apt-get update
ARG APP=/usr/src/app


EXPOSE 3000
ENV APP_USER=appuser
RUN groupadd $APP_USER \
   && useradd -g $APP_USER $APP_USER \
   && mkdir -p ${APP}


COPY --from=cargo-build /usr/src/ict/rust-medium/target/release/rust-medium ${APP}/rust-medium


RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}


CMD ["./rust-medium"]