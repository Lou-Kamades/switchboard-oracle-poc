The smart contract can be built and deployed normally using Anchor. Currently there is a devnet deployment at b36ENxZ8qYekipdAfqo7LRE1p98xy6cFNJQyM4o3sgy that the switchboard functions reference.

To deploy the function follow these steps

1. Change the DOCKERHUB_CONTAINER_NAME variable to ensure that a fresh container is used
2. `make publish_function`
3. Go to the switchboard website and fund the function, make sure that the correct measurement is included (run `make measurement` to check)
