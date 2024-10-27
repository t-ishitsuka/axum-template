.DEFAULT_GOAL := help

.PHONY: help
help:			## Makeコマンドの表示
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(firstword $(MAKEFILE_LIST)) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo ""

.PHONY: test
test:			## テスト実行
	TEST_MODE=1 cargo nextest run --workspace --status-level all --test-threads=1
