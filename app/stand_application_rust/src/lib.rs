use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};

extern crate serde;
extern crate serde_json;
extern crate serde_derive;

pub mod controllers;

pub mod common {
    use std::{
        io::{Write},
        path::Path,
        fs::{File, OpenOptions},
        string::String,
    };

    pub fn fopen(file_path: &Path, append: bool) -> File {
        let f: File = match OpenOptions::new()
            .create(true)
            .append(append)
            .write(true)
            .read(true)
            .open(file_path) {
            Err(why) => panic!("Couldn't open : {}", why),
            Ok(file) => file,
        };
        f
    }

    pub fn file_put_contents(file_path: &Path, message: String, append: bool) {
        let mut f: File = fopen(file_path, append);

        match f.write_all(message.as_bytes()) {
            Err(why) => panic!("why: {}", why),
            Ok(_) => println!("ok"),
        };
    }

    pub enum AppLogLevel {
        Debug,
        Info,
        Warning,
        Error,
    }

    pub fn write_app_log(message: &'static str, level: AppLogLevel) {
        let log_head_info = match level {
            AppLogLevel::Debug => "[DEBUG]",
            AppLogLevel::Info => "[INFO]",
            AppLogLevel::Warning => "[WARNING]",
            AppLogLevel::Error => "[ERROR]",
        };
        file_put_contents(Path::new("test.log"), format!("{}: {}\n", log_head_info, message), true);
    }
}

#[get("/")]
pub async fn get_lib_index() -> impl Responder {
    HttpResponse::Ok().body("lib test")
}

pub async fn route() -> std::io::Result<()> {
    crate::common::write_app_log("test1", crate::common::AppLogLevel::Debug);
    crate::common::write_app_log("test2", crate::common::AppLogLevel::Warning);
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(get_lib_index)
                    // ここにAPIのconfigを追加していく
                    .configure(controllers::stand_user::user::config)
            )
            // /apiから始まらないものをルーティングしたいならここに追加していく
            // index.html表示
            .route(
                "/", web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .bind("actix-web-container:8088")?
    .run()
    .await
}