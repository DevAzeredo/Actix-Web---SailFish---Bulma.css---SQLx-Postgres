use actix_web::{error, web, web::Data, HttpResponse, Responder};
use actix_web_lab::respond::Html;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Product {
    id: i64,
    descricao: String,
    valor: f64,
}
struct Pedidos {
    products: Vec<Product>,
}
/// deserialize `Info` from request's body
pub async fn enviar_pedido(products: web::Json<Vec<Product>>) -> HttpResponse  {
    let body = &products[1].descricao;
    println!("model: {:?}", &products[1].descricao);
    HttpResponse::Ok().json(&products[1].descricao)
}
