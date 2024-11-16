.DEFAULT_GOAL := help

.PHONY: help
help:			## Makeコマンドの表示
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(firstword $(MAKEFILE_LIST)) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo ""

############################################################
#
# Docker
#
############################################################
.PHONY: init
init:		## Docker コンテナ初期化
	@make network
	@make build
	@make up

.PHONY: network
network:	## Docker ネットワークの作成
	docker network create axum-template-network

.PHONY: build
build:		## Docker コンテナのビルド
	docker compose build --no-cache

.PHONY: up
up:			## Docker コンテナの開始
	docker compose up -d

.PHONY: stop
stop:		## Docker コンテナの停止
	docker compose stop

.PHONY: down
down:		## Docker コンテナの削除
	docker compose down --remove-orphans

############################################################
#
# Shell
#
############################################################
.PHONY: bash
bash: 			## Rust Server接続
	docker compose exec axum-template-backend bash

.PHONY: pg
pg: 			## PostgreSQL Server接続
	docker compose exec axum-template-pgsql bash -c 'psql -U user local'

.PHONY: redis
redis: 			## Redis Server接続
	docker compose exec axum-template-redis sh -c 'redis-cli'

############################################################
#
# Util
#
############################################################
.PHONY: test
test:			## テスト実行
	docker compose exec axum-template-backend bash -c 'TEST_MODE=1 cargo nextest run --workspace --status-level all --test-threads=1'
