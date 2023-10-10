The smart contract can be built and deployed normally using Anchor. Currently there is a devnet deployment at 7zNxbvdozQr5zmg6fX3ZpZhWGtoCpUvpSxHXvC25gSWS that the switchboard functions reference.

To deploy the switchboard function follow these steps:

1. Change the DOCKERHUB_CONTAINER_NAME variable to ensure that a fresh container is used
2. `make publish_function`
3. Go to the switchboard website and fund the function, make sure that the correct measurement is included (run `make measurement` to check)

To add an oracle to the contract:

1. Ensure the programId and ORACLE_NAME variables are set properly in `scripts/addOracle.ts`
2. `yarn addOracle`

To run Anchor tests:
`anchor test -- --features "test"`
