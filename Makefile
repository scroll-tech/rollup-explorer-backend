.PHONY: docker

docker:
	docker-compose -f docker-compose.yml build

lint:
	cd backend && cargo fmt --all && cargo clippy -- -D warnings

start:
	./scripts/stop.sh
	./scripts/start.sh

stop:
	./scripts/stop.sh

shfmt:
	shfmt -i 2 -sr -w scripts/*.sh
