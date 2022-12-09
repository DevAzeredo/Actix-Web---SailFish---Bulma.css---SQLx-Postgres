use crate::AppState;
use actix_web::{error, web::Data, Responder};
use actix_web_lab::respond::Html;
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(TemplateOnce)]
#[template(path = "pagina_home.stpl")]
struct Home {
    products: Vec<Product>,
}

pub async fn home(pool: Data<AppState>) -> actix_web::Result<impl Responder> {
    let hom = Home {
        products: get_all_products(pool).await,
    };
    let body = hom.render_once().map_err(error::ErrorInternalServerError)?;
    Ok(Html(body))
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Product {
    id: i64,
    descricao: String,
    valor: f64,
}

pub async fn get_all_products(pool: Data<AppState>) -> Vec<Product> {
    let res = sqlx::query_as::<_, Product>("SELECT id, descricao, valor FROM produto")
        .fetch_all(&pool.db)
        .await
        .unwrap();
    res
}
