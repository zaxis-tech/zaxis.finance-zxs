FROM debian:buster-slim

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG IMAGE_NAME

LABEL io.parity.image.authors="devops-team@parity.io" \
	io.parity.image.vendor="Parity Technologies" \
	io.parity.image.title="${IMAGE_NAME}" \
	io.parity.image.description="zaxis: a platform for web3" \
	io.parity.image.source="https://github.com/paritytech/zaxis/blob/${VCS_REF}/scripts/docker/Dockerfile" \
	io.parity.image.revision="${VCS_REF}" \
	io.parity.image.created="${BUILD_DATE}" \
	io.parity.image.documentation="https://github.com/paritytech/zaxis/"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
		libssl1.1 \
		ca-certificates \
		curl && \
# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
# add user and link ~/.local/share/zaxis to /data
	useradd -m -u 1000 -U -s /bin/sh -d /zaxis zaxis && \
	mkdir -p /data /zaxis/.local/share && \
	chown -R zaxis:zaxis /data && \
	ln -s /data /zaxis/.local/share/zaxis

# add zaxis binary to docker image
COPY ./zaxis /usr/local/bin

USER zaxis

# check if executable works in this container
RUN /usr/local/bin/zaxis --version

EXPOSE 30333 9933 9944
VOLUME ["/zaxis"]

ENTRYPOINT ["/usr/local/bin/zaxis"]
