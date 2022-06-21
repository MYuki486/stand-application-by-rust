# スタンドの非公式ツール群

## 起動

`docker-compose up -d --build`

## API

[RUST](https://www.rust-lang.org/) + [actix-web](https://actix.rs/)

M(V)C方式で開発 ([actix-web](https://actix.rs/)公式対応してない)

*メイン* `./app/stand_application_rust/src/main.rs`

*ルーティング* `./app/stand_application_rust/src/lib.rs`

*コントローラー* `./app/stand_application_rust/src/controllers/*/*.rs`(mod.rsを除く)

### controller routing ex)

`./app/stand_application_rust/src/controllers/stand_user/user.rs`にAPI配置したい

*user.rs*
```rs
// 編集の影響範囲減らしたいからconfig設定
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // lib.rsのscopeに影響を受ける(localhost/api/)
        // localhost/api/user
        web::scope("/user")
            .service(get_index)
            .service(get_list),
    );
}

// localhost/api/user/
#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("user index")
}

// localhost/api/user/list
#[get("/list")]
async fn get_list() -> impl Responder {
    HttpResponse::Ok().body("user list")
}
```

*lib.rs*
```rs
// ルーティング
pub async fn route() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                // localhost/api
                web::scope("/api")
                    .configure(
                        // コントローラーで作成したconfig読み込み
                        controllers::stand_user::user::config
                    )
                    // コントローラー追加したらここに追加記述
            )
            ...
    })
...
```

## フロントエンド (予定)

### 開発言語

[TypeScript](https://www.typescriptlang.org/)

### フレームワーク

[Svelte](https://svelte.jp/)

