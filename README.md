# axum-template

### ディレクトリ構成

```
project-root/
  ├ src/bin/app.rs
  ├ shares/
  ├   ├ env/
  │   ├ src/
  │   │   └ config/
  │   └ tests/
  ├ interfaces/
  │   └ src/
  │       └ rest/
  │           ├ controllers
  │           │   └ v1
  │           ├ requests
  │           ├ responses
  │           ├ routes
  │           │   └ v1
  │           └ schemas
  ├ usecases/
  ├ domains/
  └ infrastructures/
```

[ディレクトリ構成参考](https://qiita.com/tono-maron/items/345c433b86f74d314c8d)

### 進捗

-   [x] hello world
-   [x] health check
-   [x] nested routing
-   [x] config, load env, load .env
-   [x] .env.testing
-   [x] app env config
-   [x] test1: config
-   [] swagger gen
    -   [x] health check path
    -   [x] response
    -   [x] tags
    -   [] request
    -   [] path params
    -   [] query params
    -   [] json or yaml の書き出し
-   [x] add rust docker
-   [] rust docker の cargo watch が遅いのを修正したい
-   [x] add postgresql docker
-   [x] create database
-   [x] add redis docker
-   [] Makefile
-   [] add usecase layer
-   [] dependency injection
-   [] add test authed user endpoint
-   [] test
-   [] add ORM
-   [] migration (users)
-   [] update authed user endpoint (no jwt)
-   [] test
-   [] add jwt crait
-   [] add login
-   [] test
-   [] update authed user endpoint (with jwt)
-   [] test カバレッジの取得
