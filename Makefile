.PHONY: docker

docker:
	docker-compose -f docker-compose.yml build ## TODO: add frontend

lint:
	cd backend && cargo fmt --all && cargo clippy -- -D warnings
	cd frontend && yarn install && yarn run lint

start_backend:
	./scripts/stop.sh
	./scripts/start.sh

start_frontend:
	cd frontend && yarn install && yarn run dev

stop:
	./scripts/stop.sh

shfmt:
	shfmt -i 2 -sr -w scripts/*.sh
