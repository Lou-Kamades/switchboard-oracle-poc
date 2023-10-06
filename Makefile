# include .env file and export its env vars
# (-include to ignore error if it does not exist)
-include .env

.PHONY: build clean publish test

check_docker_org:
ifeq ($(strip $(DOCKERHUB_ORGANIZATION)),)
	$(error DOCKERHUB_ORGANIZATION is not set)
else
	@echo DOCKERHUB_ORGANIZATION: ${DOCKERHUB_ORGANIZATION}
endif

check_docker_name:
ifeq ($(strip $(DOCKERHUB_CONTAINER_NAME)),)
	$(error DOCKERHUB_CONTAINER_NAME is not set)
else
	@echo DOCKERHUB_CONTAINER_NAME: ${DOCKERHUB_CONTAINER_NAME}
endif

# Default make task
all: anchor_sync build

anchor_sync :; anchor keys sync
anchor_build :; anchor build
anchor_publish:; make oracle_deploy

docker_build_rust: check_docker_org check_docker_name
	docker buildx build --platform linux/amd64 --pull -f ./Dockerfile.rust -t ${DOCKERHUB_ORGANIZATION}/${DOCKERHUB_CONTAINER_NAME} --load .
docker_publish_rust: check_docker_org check_docker_name
	docker buildx build --platform linux/amd64 --pull -f ./Dockerfile.rust -t ${DOCKERHUB_ORGANIZATION}/${DOCKERHUB_CONTAINER_NAME} --push .

measurement: check_docker_org check_docker_name
	docker pull --platform=linux/amd64 -q ${DOCKERHUB_ORGANIZATION}/${DOCKERHUB_CONTAINER_NAME}:latest
	@docker run -d --platform=linux/amd64 -q --name=poc-switchboard-oracle  ${DOCKERHUB_ORGANIZATION}/${DOCKERHUB_CONTAINER_NAME}:latest
	@docker cp poc-switchboard-oracle:/measurement.txt ./measurement.txt
	@echo -n 'MrEnclave: '
	@cat measurement.txt
	@docker stop poc-switchboard-oracle > /dev/null
	@docker rm poc-switchboard-oracle > /dev/null

oracle_deploy:
	anchor build -p fat_oracle
	anchor deploy --provider.cluster devnet -p fat_oracle --program-keypair ${ANCHOR_WALLET}

# Task to clean up the compiled rust application
clean:
	cargo clean

	
