.PHONY: all init_db init_redis init_mailhog run

all: init_db init_redis init_mailhog run

init_db:
	./scripts/init_db.sh

init_redis:
	./scripts/init_redis.sh

init_mailhog:
	./scripts/init_mailhog.sh

run: init_db init_redis init_mailhog
	export $$(cat .env | xargs) && cargo run --bin app
