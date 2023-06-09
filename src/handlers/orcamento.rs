use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::orcamento::{create, delete, find, NewOrcamento, Orcamento};
use crate::models::orcamento_produto::{create as create_orcamento_produto, 
    delete as delete_orcamento_produto, 
    find as find_orcamento_produto, NewOrcamentoProduto, OrcamentoProduto};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use std::thread;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OrcamentoResponse {
    pub id: Uuid,
    pub quantidade: i32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OrcamentosResponse(pub Vec<OrcamentoResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateOrcamentoRequest {
    produtos: Vec<OrcamentoProduto>,
}


/// Get a orcamento
pub async fn get_orcamento(
    orcamento_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<Json<OrcamentoResponse>, ApiError> {
    let orcamento = block(move || find(&pool, *orcamento_id)).await?;
    respond_json(orcamento)
}

/// Create a orcamento
pub async fn create_orcamento(
    pool: Data<PoolType>,
    params: Json<CreateOrcamentoRequest>,
) -> Result<Json<OrcamentoResponse>, ApiError> {
    validate(&params)?;
    let orcamento_id = Uuid::new_v4();
    let new_orcamento: Orcamento = NewOrcamento {
        id: orcamento_id.to_string(),
        created_by: orcamento_id.to_string(),
    }
    .into();
    let new_pool = pool.clone();
    let orcamento = block(move || create(&pool, &new_orcamento)).await?;
    // temporarily use the new orcamento's id for created_at/updated_at
    // update when auth is added
    params.produtos.iter().for_each(|params|{
        let new_pool = new_pool.clone();
        let orcamento_produto_id = Uuid::new_v4();
        let new_orcamento_produto: OrcamentoProduto = NewOrcamentoProduto {
            id: orcamento_produto_id.to_string(),
            id_produto: params.id_produto.to_string(),
            id_orcamento: params.id_orcamento.to_string(),
            quantidade: params.quantidade,
            created_by: orcamento_produto_id.to_string(),
        }
        .into();
        create_orcamento_produto(&new_pool, &new_orcamento_produto);
    });

    
    respond_json(orcamento.into())
}

/// Delete a orcamento
pub async fn delete_orcamento(
    orcamento_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<HttpResponse, ApiError> {
    block(move || delete(&pool, *orcamento_id)).await?;
    respond_ok()
}

impl From<Orcamento> for OrcamentoResponse {
    fn from(orcamento: Orcamento) -> Self {
        OrcamentoResponse {
            id: Uuid::parse_str(&orcamento.id).unwrap(),
            quantidade: 1i32,
        }
    }
}

impl From<Vec<Orcamento>> for OrcamentosResponse {
    fn from(orcamentos: Vec<Orcamento>) -> Self {
        OrcamentosResponse(orcamentos.into_par_iter().map(|orcamento| orcamento.into()).collect())
    }
}

///Testes
///Testes
///Testes
#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::models::orcamento::tests::create_orcamento as model_create_orcamento;
    use crate::models::orcamento::delete as model_delete;
    use crate::tests::helpers::tests::{get_data_pool, get_pool};

    #[actix_rt::test]
    async fn it_gets_a_orcamento() {
        let orcamento_id = Path::from(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
        let response = get_orcamento(orcamento_id, get_data_pool()).await;
        assert!(response.is_ok());
    }

    #[actix_rt::test]
    async fn it_doesnt_find_a_orcamento() {
        let uuid = Uuid::new_v4();
        let orcamento_id: Path<Uuid> = uuid.into();
        let response = get_orcamento(orcamento_id, get_data_pool()).await;
        let expected_error = ApiError::NotFound(format!("Orcamento {} not found", uuid.to_string()));
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_error);
    }

    #[actix_rt::test]
    async fn it_creates_a_orcamento() {
        let orcamento_produto: OrcamentoProduto = NewOrcamentoProduto{
            id: Uuid::new_v4().to_string(),
            id_produto: "00000000-0000-0000-0000-000000000000".into(),
            id_orcamento: "00000000-0000-0000-0000-000000000000".into(),
            quantidade: 1,
            created_by: "00000000-0000-0000-0000-000000000000".into(),
        }.into();
        let orcamentos_produtos = vec![orcamento_produto];
        let params = Json(CreateOrcamentoRequest {
            produtos: orcamentos_produtos,
        });
        let response = create_orcamento(get_data_pool(), Json(params.clone()))
            .await;
        assert!(response.is_ok());
        delete(&get_data_pool(), response.unwrap().id);
    }

    #[actix_rt::test]
    async fn it_deletes_a_orcamento() {
        let created = model_create_orcamento();
        let orcamento_id = created.unwrap().id;
        let orcamento_id_path: Path<Uuid> = orcamento_id.into();
        let orcamento = find(&get_pool(), orcamento_id);
        assert!(orcamento.is_ok());
        delete_orcamento(orcamento_id_path, get_data_pool()).await.unwrap();
        let orcamento = find(&get_pool(), orcamento_id);
        assert!(orcamento.is_err());
    }
}
