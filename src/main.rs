use actix_files::NamedFile;
use actix_web::{
    middleware::{Compress, Logger},
    web,
    web::Data,
    App, HttpRequest, HttpServer, Result,
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::path::PathBuf;
mod home;
mod pedidos;

pub struct AppState {
    db: Pool<Postgres>,
}

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
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
            .app_data(web::JsonConfig::default().limit(4096))
            .service(
                web::resource("/")
                    .app_data(pool.clone())
                    .route(web::get().to(home::home)),
            )
            .service(
                web::resource("/EnviarPedido")
                    .route(web::post().to(pedidos::enviar_pedido)),
            )
            .route("/{bulma.min:.css}", web::get().to(index))
            .wrap(Compress::default())
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
