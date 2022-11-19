use actix_web::{
    middleware::{Compress, Logger},
    web,
    web::Data,
    App, HttpServer,HttpRequest, Error, get,
};
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_files as fs;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
mod home;

pub struct AppState {
    db: Pool<Postgres>,
}
async fn get_response(link: String) -> serde_json::Value {
    // create client
    let client = awc::Client::default();

    // construct request
    let req = client.get(link).insert_header(("User-Agent", "awc/3.0"));

    // send request and await response
    let mut response = req.send().await.unwrap();
    log::info!("Response: {:?}", response);

    let value = response.json::<serde_json::Value>().await.unwrap();

    value
}
/*Com essa função dá pra entregar outros arquivos também, basta alterar o nome de bulma.min pra filename e o :.css pra :.* e em query(bulma.min pra filename) */
#[get("/{bulma.min:.css}")]
async fn index(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info().query("bulma.min").parse().unwrap();
    let file = fs::NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");
    // tabela_produto::get_all_produtos(&pool).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(
                web::resource("/")
                    .app_data(pool.clone())
                    .route(web::get().to(home::home)),
            )
            .service(index)
            .wrap(Compress::default())
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
