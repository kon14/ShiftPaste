FROM rust:1.75-alpine as build
RUN apk add musl-dev curl
WORKDIR /app
COPY . .
ARG BUILD_FEATURES=""
RUN cargo build --release --features "$BUILD_FEATURES"

FROM alpine:3.19 as app
COPY --from=build /app/target/release/ShiftPaste /
ENV RUST_LOG="error"
ENV DATABASE_URL=
ENV API_PORT=4000
ENV API_BASE_URL=
ENV APP_SNIPPET_VIEW_URL=
ENV AUTH_JWT_SECRET=
ENV AUTH_ACCESS_TOKEN_DURATION_SECS=
ENV AUTH_REFRESH_TOKEN_DURATION_SECS=

EXPOSE 4000
CMD ["/ShiftPaste"]
