.PHONY: help build fmt lint test run actor-id clean web-build web-test game-build game-dev game-stack game-stop e2e-test

help:
	@printf "Available targets:\n"
	@printf "  build  - Build the backend\n"
	@printf "  fmt    - Format Rust sources\n"
	@printf "  lint   - Run clippy with warnings denied\n"
	@printf "  test   - Run unit and integration tests\n"
	@printf "  run    - Start the backend locally\n"
	@printf "  web-build - Build the admin and game frontend bundles\n"
	@printf "  web-test  - Run lightweight frontend smoke tests\n"
	@printf "  game-build - Build only the React game frontend\n"
	@printf "  game-dev   - Start the React game frontend dev server\n"
	@printf "  game-stack - Start backend and React game frontend together\n"
	@printf "  game-stop  - Stop stale backend and React game frontend processes\n"
	@printf "  e2e-test   - Run browser E2E for the React game frontend\n"
	@printf "  actor-id - Create a demo profile and print its id for X-Actor-Id\n"
	@printf "  clean  - Remove build artifacts\n"

build: web-build
	cargo build

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test: web-test
	cargo test -j 1

run: web-build
	cargo run --bin x10-backend

web-build:
	./web/build.sh

web-test:
	./web/test.sh

game-build:
	./web/game/build.sh

game-dev:
	./web/game/dev.sh

game-stack:
	./web/scripts/game_dev_stack.sh

game-stop:
	./web/scripts/kill_game_processes.sh

e2e-test: web-build
	cd web && npm run e2e:game

actor-id:
	@response=$$(curl -fsS -X POST http://127.0.0.1:$${X10_PORT:-3000}/api/v2/profiles \
		-H 'content-type: application/json' \
		-d '{"full_name":"Docs Demo","birth_date":"1990-01-01","occupation":"tester","timezone":"Europe/Samara"}'); \
	printf '%s\n' "$$response"; \
	printf '%s\n' "$$response" | sed -n 's/.*"id":"\([0-9a-fA-F-]\{36\}\)".*/Use X-Actor-Id: \1/p'

clean:
	cargo clean
