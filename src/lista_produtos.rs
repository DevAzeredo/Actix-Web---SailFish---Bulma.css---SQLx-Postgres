use crate::AppState;
use actix_web::{error, web::Data, Responder};
use actix_web_lab::respond::Html;
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(TemplateOnce)]
#[template(path = "lista_produtos.stpl")]
struct lista_produtos {
    produtos: Vec<Produto>,
}

pub async fn lista_produtos(pool: Data<AppState>) -> actix_web::Result<impl Responder> {
    let hom = lista_produtos {
        produtos: get_all_produtos(pool).await,
    };
    let body = hom.render_once().map_err(error::ErrorInternalServerError)?;
    Ok(Html(body))
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Produto {
    id: i64,
    descricao: String,
    valor: f64,
}

pub async fn get_all_produtos(pool: Data<AppState>) -> Vec<Produto> {
    let res = sqlx::query_as::<_, Produto>("SELECT id, descricao, valor FROM produto")
        .fetch_all(&pool.db)
        .await
        .unwrap();
    res
}
