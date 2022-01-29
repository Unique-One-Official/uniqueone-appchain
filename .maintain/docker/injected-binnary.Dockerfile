FROM ubuntu:20.04
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
		libssl1.1 \
		ca-certificates && \
  # apt cleanup
    apt-get autoremove -y && \
    apt-get clean && \
    find /var/lib/apt/lists/ -type f -not -name lock -delete; \
  # Create user and set ownership and permissions as required
  useradd -m -u 1001 -U -s /bin/sh -d /home/uniqueone-appchain uniqueone-appchain && \
  # manage folder data
  mkdir -p /home/uniqueone-appchain/.local/share && \
  mkdir /data && \
  chown -R uniqueone-appchain:uniqueone-appchain /data && \
  ln -s /data /home/uniqueone-appchain/.local/share/uniqueone-appchain
# Add binnary to docker image
COPY ./uniqueone-appchain /usr/local/bin
# Set to a non-root built-in user
USER uniqueone-appchain
# Set environment variable
ENV RUST_BACKTRACE=1
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/uniqueone-appchain"]
