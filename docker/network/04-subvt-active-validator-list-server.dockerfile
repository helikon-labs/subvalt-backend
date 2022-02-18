FROM helikon/subvt-backend-base:0.1.2 as builder

FROM debian:buster-slim
# install certificate authority certificates, create config directory
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && update-ca-certificates \
    && mkdir -p /subvt/config
# copy config files
COPY ./subvt-config/config/ /subvt/config/
# copy executable
COPY --from=builder /subvt/bin/subvt-validator-list-server /usr/local/bin/
CMD ["subvt-validator-list-server"]