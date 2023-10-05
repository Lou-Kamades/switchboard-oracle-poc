# include .env file and export its env vars
# (-include to ignore error if it does not exist)
-include .env

.PHONY: build clean publish test

# Variables
DOCKER_IMAGE_NAME=loukamades/pong
DOCKERHUB_IMAGE_NAME=loukamades/pong

check_docker_env:
ifeq ($(strip $(DOCKERHUB_IMAGE_NAME)),)
	$(error DOCKERHUB_IMAGE_NAME is not set)
else
	@echo DOCKERHUB_IMAGE_NAME: ${DOCKERHUB_IMAGE_NAME}
endif

# Default make task
all: anchor_sync build

anchor_sync :; anchor keys sync
anchor_build :; anchor build
anchor_publish:; make oracle_deploy

docker_build_rust: 
	docker buildx build --platform linux/amd64 --pull -f ./Dockerfile.rust -t ${DOCKER_IMAGE_NAME} --load .
docker_publish_rust: 
	docker buildx build --platform linux/amd64 --pull -f ./Dockerfile.rust -t ${DOCKER_IMAGE_NAME} --push .

docker_build_ts: 
	docker buildx build --platform linux/amd64 --pull -f ./Dockerfile.ts -t ${DOCKER_IMAGE_NAME} --load .
docker_publish_ts: 
	docker buildx build --platform linux/amd64 --pull -f ./Dockerfile.ts -t ${DOCKER_IMAGE_NAME} --push .

# anchor, docker, measurement

measurement: check_docker_env
	docker pull --platform=linux/amd64 -q ${DOCKERHUB_IMAGE_NAME}:latest
	@docker run -d --platform=linux/amd64 -q --name=poc-switchboard-oracle ${DOCKERHUB_IMAGE_NAME}:latest
	@docker cp poc-switchboard-oracle:/measurement.txt ./measurement.txt
	@echo -n 'MrEnclave: '
	@cat measurement.txt
	@docker stop poc-switchboard-oracle > /dev/null
	@docker rm poc-switchboard-oracle > /dev/null

oracle_deploy:
	anchor build -p fat_oracle
	anchor deploy --provider.cluster devnet -p fat_oracle --program-keypair ${KEYPAIR_PATH}

# Task to clean up the compiled rust application
clean:
	cargo clean

	
