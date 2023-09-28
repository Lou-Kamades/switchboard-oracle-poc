# syntax=docker/dockerfile:1.4
FROM switchboardlabs/sgx-function AS builder


# TODO: enable building once the TS package is fixed
WORKDIR /home/root/switchboard-function
# COPY ./function-ts/package.json ./function-ts/tsconfig.json ./function-ts/package-lock.json ./function-ts/node_modules/ ./
# RUN npm install
# COPY ./function-ts/src/ ./src/
# RUN npm run build
# RUN cp ./function-ts/dist/index.js /sgx/nodejs/index.js
COPY ./function-ts/dist/index.js /sgx/nodejs/index.js

FROM switchboardlabs/sgx-function

# Copy the binary
WORKDIR /sgx
COPY --from=builder /sgx/nodejs/index.js /sgx/nodejs/index.js

# Get the measurement from the enclave
RUN rm -f /measurement.txt && \
    /get_measurement.sh --nodejs && \
    cat /measurement.txt

ENTRYPOINT ["bash", "/boot.sh", "--nodejs"]