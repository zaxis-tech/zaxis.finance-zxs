FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Z-Axis. Here we create the binary."

ARG PROFILE=release
WORKDIR /zaxis

COPY . /zaxis

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the Z-Axis binary."
ARG PROFILE=release
COPY --from=builder /zaxis/target/$PROFILE/zaxis /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /zaxis zaxis && \
	mkdir -p /zaxis/.local/share && \
	mkdir /data && \
	chown -R zaxis:zaxis /data && \
	ln -s /data /zaxis/.local/share/zaxis && \
	rm -rf /usr/bin /usr/sbin

USER zaxis
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/zaxis"]
