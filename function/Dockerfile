# syntax=docker/dockerfile:1.4
FROM switchboardlabs/sgx-function AS builder

WORKDIR /home/root/switchboard-function
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src/
# May need to pre build rust? 
# There are issues with the anchor program Cargo dependency if you use path in the Cargo.toml
#COPY ./target/release /sgx/app

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} \
    --mount=type=cache,target=target,id=${TARGETPLATFORM} \
    cargo build --release && \
    cargo strip && \
    mv target/release/function /sgx/app

FROM switchboardlabs/sgx-function

# Copy the binary
WORKDIR /sgx
COPY --from=builder /sgx/app /sgx

# Get the measurement from the enclave
RUN rm -f /measurement.txt
RUN /get_measurement.sh && \
    cat /measurement.txt

ENTRYPOINT ["bash", "/boot.sh"]
