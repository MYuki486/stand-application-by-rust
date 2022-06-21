extern crate stand_application_rust;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    stand_application_rust::route().await
}
