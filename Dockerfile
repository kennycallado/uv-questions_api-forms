FROM busybox:latest

COPY ./target/x86_64-unknown-linux-musl/release/q-api-forms /bin/q-api-forms
COPY ./Rocket.toml /root

WORKDIR /root

CMD [ "q-api-forms" ]
