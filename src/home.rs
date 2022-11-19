use crate::AppState;
use actix_web::{error, web::Data, Responder};
use actix_web_lab::respond::Html;
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
struct Home {
    produtos: Vec<Produto>,
}

pub async fn home(pool: Data<AppState>) -> actix_web::Result<impl Responder> {
    let hom = Home {
        produtos: get_all_produtos(pool).await,
    };
    let body = hom.render_once().map_err(error::ErrorInternalServerError)?;
    Ok(Html(body))
}

/*async fn get_id(p_produto: Vec<Pessoas>) -> Home {
    let mut hom: Home;
    let mut i = 0;
    hom = Home {
        id: Vec::new(),
        nome: Vec::new(),
    };

    for pes in p_produto {
        log::info!("tamanho id:{:?}", hom.id.len());

        hom.id.insert(i, pes.id);
        log::info!("tamanho nome:{:?}", hom.nome.len());
        hom.nome.insert(i, pes.name);
        i = i + 1;
    }
    hom
}*/

//let pas: Vec<Pessoas> = serde_json::from_value(a).unwrap();
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
