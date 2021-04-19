A [webring++](https://github.com/webring-plusplus) server implementation for [actix-web](https://lib.rs/crates/actix-web).

```
use webring_plusplus_server_actix::webring_plusplus_service;

#[actix_rt::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new().service(webring_plusplus_service(vec![
            Cow::from("askjeeves.com"),
            Cow::from("altavista.com"),
        ]))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
```
