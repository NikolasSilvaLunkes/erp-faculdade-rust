use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::orcamento_produto::{create, delete, find, NewOrcamentoProduto, OrcamentoProduto};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OrcamentoProdutoResponse {
    pub id: Uuid,
    pub id_produto: Uuid,
    pub id_orcamento: Uuid,
    pub quantidade: i32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OrcamentoProdutosResponse(pub Vec<OrcamentoProdutoResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateOrcamentoProdutoRequest {
    pub id_produto: Uuid,
    pub id_orcamento: Uuid,
    pub quantidade: i32,
}


/// Get a orcamento_produto
pub async fn get_orcamento_produto(
    orcamento_produto_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<Json<OrcamentoProdutoResponse>, ApiError> {
    let orcamento_produto = block(move || find(&pool, *orcamento_produto_id)).await?;
    respond_json(orcamento_produto)
}

/// Create a orcamento_produto
pub async fn create_orcamento_produto(
    pool: Data<PoolType>,
    params: Json<CreateOrcamentoProdutoRequest>,
) -> Result<Json<OrcamentoProdutoResponse>, ApiError> {
    validate(&params)?;

    // temporarily use the new orcamento_produto's id for created_at/updated_at
    // update when auth is added
    let orcamento_produto_id = Uuid::new_v4();
    let new_orcamento_produto: OrcamentoProduto = NewOrcamentoProduto {
        id: orcamento_produto_id.to_string(),
        id_produto: params.id_produto.to_string(),
        id_orcamento: params.id_orcamento.to_string(),
        quantidade: params.quantidade,
        created_by: orcamento_produto_id.to_string(),
    }
    .into();
    let orcamento_produto = block(move || create(&pool, &new_orcamento_produto)).await?;
    respond_json(orcamento_produto.into())
}

/// Delete a orcamento_produto
pub async fn delete_orcamento_produto(
    orcamento_produto_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<HttpResponse, ApiError> {
    block(move || delete(&pool, *orcamento_produto_id)).await?;
    respond_ok()
}

impl From<OrcamentoProduto> for OrcamentoProdutoResponse {
    fn from(orcamento_produto: OrcamentoProduto) -> Self {
        OrcamentoProdutoResponse {
            id: Uuid::parse_str(&orcamento_produto.id).unwrap(),
            id_produto: Uuid::parse_str(&orcamento_produto.id_produto).unwrap(),
            id_orcamento: Uuid::parse_str(&orcamento_produto.id_orcamento).unwrap(),
            quantidade: 1i32,
        }
    }
}

impl From<Vec<OrcamentoProduto>> for OrcamentoProdutosResponse {
    fn from(orcamento_produtos: Vec<OrcamentoProduto>) -> Self {
        OrcamentoProdutosResponse(orcamento_produtos.into_par_iter().map(|orcamento_produto| orcamento_produto.into()).collect())
    }
}

///Testes
///Testes
///Testes
#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::models::orcamento_produto::tests::create_orcamento_produto as model_create_orcamento_produto;
    use crate::models::orcamento_produto::delete as model_delete;
    use crate::tests::helpers::tests::{get_data_pool, get_pool};

    #[actix_rt::test]
    async fn it_gets_a_orcamento_produto() {
        let orcamento_produto_id = Path::from(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
        let response = get_orcamento_produto(orcamento_produto_id, get_data_pool()).await;
        assert!(response.is_ok());
    }

    #[actix_rt::test]
    async fn it_doesnt_find_a_orcamento_produto() {
        let uuid = Uuid::new_v4();
        let orcamento_produto_id: Path<Uuid> = uuid.into();
        let response = get_orcamento_produto(orcamento_produto_id, get_data_pool()).await;
        let expected_error = ApiError::NotFound(format!("OrcamentoProduto {} not found", uuid.to_string()));
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_error);
    }

    #[actix_rt::test]
    async fn it_creates_a_orcamento_produto() {
        let params = Json(CreateOrcamentoProdutoRequest {
            id_produto: Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
            id_orcamento: Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
            quantidade: 1i32,
        });
        let response = create_orcamento_produto(get_data_pool(), Json(params.clone()))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(response.quantidade, params.quantidade);
        delete(&get_data_pool(), response.id);
    }

    #[actix_rt::test]
    async fn it_deletes_a_orcamento_produto() {
        let created = model_create_orcamento_produto();
        let orcamento_produto_id = created.unwrap().id;
        let orcamento_produto_id_path: Path<Uuid> = orcamento_produto_id.into();
        let orcamento_produto = find(&get_pool(), orcamento_produto_id);
        assert!(orcamento_produto.is_ok());
        delete_orcamento_produto(orcamento_produto_id_path, get_data_pool()).await.unwrap();
        let orcamento_produto = find(&get_pool(), orcamento_produto_id);
        assert!(orcamento_produto.is_err());
    }
}
