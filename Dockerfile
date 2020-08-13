FROM ekidd/rust-musl-builder:latest as build

# Install capnproto
RUN curl -O https://capnproto.org/capnproto-c++-0.8.0.tar.gz;\
		tar zxf capnproto-c++-0.8.0.tar.gz;\
		cd capnproto-c++-0.8.0;\
		./configure;\
		make -j6 check;\
		sudo make install

#Cache dependencies
COPY Cargo.* ./
COPY build.rs ./build.rs
COPY protos/ ./protos/
RUN mkdir src; \
		echo "fn main() {}" > src/main.rs; \
		cargo build --release --target x86_64-unknown-linux-musl

# Build the application proper
COPY src/ src/

RUN sudo touch src/main.rs;\
		cargo build --release --target x86_64-unknown-linux-musl;\
		strip target/x86_64-unknown-linux-musl/release/queryer

# Runtime image
FROM scratch
COPY --from=build /etc/ssl/certs/ /etc/ssl/certs/
COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/queryer .
CMD [ "./queryer" ]
