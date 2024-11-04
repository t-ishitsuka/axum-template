.DEFAULT_GOAL := help

.PHONY: help
help:			## Makeコマンドの表示
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(firstword $(MAKEFILE_LIST)) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo ""

.PHONY: init
init:		## Docker コンテナ初期化
	@make network
# @make build
# @make up

.PHONY: build
build:		## Docker コンテナのビルド
	docker compose build --no-cache

.PHONY: up
up:			## Docker コンテナの開始
	docker compose up -d

.PHONY: network
network:	## Docker ネットワークの作成
	docker network create axum-template-network

.PHONY: test
test:			## テスト実行
	TEST_MODE=1 cargo nextest run --workspace --status-level all --test-threads=1
