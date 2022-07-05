.PHONY: docker

docker:
	docker compose -f docker-compose.yml build --build-arg next_public_base_api_url="$(NEXT_PUBLIC_BASE_API_URL)"

lint:
	cd backend && cargo fmt --all && cargo clippy -- -D warnings
	cd frontend && yarn install && yarn run lint

start_be:
	./scripts/stop.sh
	./scripts/start_be_dev.sh

start_fe:
	cd frontend && yarn install && yarn run dev

stop:
	./scripts/stop.sh

shfmt:
	shfmt -i 2 -sr -w scripts/*.sh
