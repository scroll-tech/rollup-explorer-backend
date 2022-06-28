.PHONY: docker

docker:
	docker-compose -f docker/docker-compose.yml build

lint:
	cargo fmt --all
	cargo clippy -- -D warnings

start:
	./scripts/stop.sh
	./scripts/start.sh

stop:
	./scripts/stop.sh

shfmt:
	shfmt -i 2 -sr -w scripts/*.sh
