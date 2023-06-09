use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::produto::{create, delete, find, get_all, update, NewProduto, UpdateProduto, Produto};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use bigdecimal::BigDecimal;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ProdutoResponse {
    pub id: Uuid,
    pub descricao: String,
    pub codigo_de_barras: Option<String>,
    pub icms: Option<String>,
    pub unidade_medida: Option<String>,
    pub preco_venda: Option<BigDecimal>,
    pub preco_minimo: Option<BigDecimal>,
    pub custo_medio: Option<BigDecimal>,
    pub pis: Option<String>,
    pub cofins: Option<String>,
    pub ncm: Option<String>,
    pub cest: Option<String>,
    pub cod_ipi: Option<String>,
    pub porcentagem_ipi: Option<BigDecimal>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ProdutosResponse(pub Vec<ProdutoResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateProdutoRequest {
    #[validate(length(min = 1))]
    descricao: String,
    codigo_de_barras: Option<String>,
    icms: Option<String>,
    unidade_medida: Option<String>,
    preco_venda: Option<BigDecimal>,
    preco_minimo: Option<BigDecimal>,
    custo_medio: Option<BigDecimal>,
    pis: Option<String>,
    cofins: Option<String>,
    ncm: Option<String>,
    cest: Option<String>,
    cod_ipi: Option<String>,
    porcentagem_ipi: Option<BigDecimal>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateProdutoRequest {
    #[validate(length(min = 1))]
    descricao: String,
    codigo_de_barras: Option<String>,
    icms: Option<String>,
    unidade_medida: Option<String>,
    preco_venda: Option<BigDecimal>,
    preco_minimo: Option<BigDecimal>,
    custo_medio: Option<BigDecimal>,
    pis: Option<String>,
    cofins: Option<String>,
    ncm: Option<String>,
    cest: Option<String>,
    cod_ipi: Option<String>,
    porcentagem_ipi: Option<BigDecimal>,
}

/// Get a produto
pub async fn get_produto(
    produto_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<Json<ProdutoResponse>, ApiError> {
    let produto = block(move || find(&pool, *produto_id)).await?;
    respond_json(produto)
}

/// Get all produtos
pub async fn get_produtos(pool: Data<PoolType>) -> Result<Json<ProdutosResponse>, ApiError> {
    let produtos = block(move || get_all(&pool)).await?;
    respond_json(produtos)
}

/// Create a produto
pub async fn create_produto(
    pool: Data<PoolType>,
    params: Json<CreateProdutoRequest>,
) -> Result<Json<ProdutoResponse>, ApiError> {
    validate(&params)?;

    // temporarily use the new produto's id for created_at/updated_at
    // update when auth is added
    let produto_id = Uuid::new_v4();
    let new_produto: Produto = NewProduto {
        id: produto_id.to_string(),
        descricao: params.descricao.clone(),
        codigo_de_barras: params.codigo_de_barras.clone(),
        icms: params.icms.clone(),
        unidade_medida: params.unidade_medida.clone(),
        preco_venda: params.preco_venda.clone(),
        preco_minimo: params.preco_minimo.clone(),
        custo_medio: params.custo_medio.clone(),
        pis: params.pis.clone(),
        cofins: params.cofins.clone(),
        ncm: params.ncm.clone(),
        cest: params.cest.clone(),
        cod_ipi: params.cod_ipi.clone(),
        porcentagem_ipi: params.porcentagem_ipi.clone(),
        created_by: produto_id.to_string(),
        updated_by: produto_id.to_string(),
    }
    .into();
    let produto = block(move || create(&pool, &new_produto)).await?;
    respond_json(produto.into())
}

/// Update a produto
pub async fn update_produto(
    produto_id: Path<Uuid>,
    pool: Data<PoolType>,
    params: Json<UpdateProdutoRequest>,
) -> Result<Json<ProdutoResponse>, ApiError> {
    validate(&params)?;

    // temporarily use the produto's id for updated_at
    // update when auth is added
    let update_produto = UpdateProduto {
        id: produto_id.to_string(),
        descricao: params.descricao.clone(),
        codigo_de_barras: params.codigo_de_barras.clone(),
        icms: params.icms.clone(),
        unidade_medida: params.unidade_medida.clone(),
        preco_venda: params.preco_venda.clone(),
        preco_minimo: params.preco_minimo.clone(),
        custo_medio: params.custo_medio.clone(),
        pis: params.pis.clone(),
        cofins: params.cofins.clone(),
        ncm: params.ncm.clone(),
        cest: params.cest.clone(),
        cod_ipi: params.cod_ipi.clone(),
        porcentagem_ipi: params.porcentagem_ipi.clone(),
        updated_by: produto_id.to_string(),
    };
    let produto = block(move || update(&pool, &update_produto)).await?;
    respond_json(produto.into())
}

/// Delete a produto
pub async fn delete_produto(
    produto_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<HttpResponse, ApiError> {
    block(move || delete(&pool, *produto_id)).await?;
    respond_ok()
}

impl From<Produto> for ProdutoResponse {
    fn from(produto: Produto) -> Self {
        ProdutoResponse {
            id: Uuid::parse_str(&produto.id).unwrap(),
            descricao: produto.descricao.clone(),
            codigo_de_barras: produto.codigo_de_barras.clone(),
            icms: produto.icms.clone(),
            unidade_medida: produto.unidade_medida.clone(),
            preco_venda: produto.preco_venda.clone(),
            preco_minimo: produto.preco_minimo.clone(),
            custo_medio: produto.custo_medio.clone(),
            pis: produto.pis.clone(),
            cofins: produto.cofins.clone(),
            ncm: produto.ncm.clone(),
            cest: produto.cest.clone(),
            cod_ipi: produto.cod_ipi.clone(),
            porcentagem_ipi: produto.porcentagem_ipi.clone(),
        }
    }
}

impl From<Vec<Produto>> for ProdutosResponse {
    fn from(produtos: Vec<Produto>) -> Self {
        ProdutosResponse(produtos.into_par_iter().map(|produto| produto.into()).collect())
    }
}

///Testes
///Testes
///Testes
#[cfg(test)]
pub mod tests {
    use bigdecimal::FromPrimitive;

    use super::*;
    use crate::models::produto::tests::create_produto as model_create_produto;
    use crate::models::produto::tests::create_produto_by_email as model_create_produto_by_email;
    use crate::models::produto::delete as model_delete;
    use crate::tests::helpers::tests::{get_data_pool, get_pool};

    pub fn get_all_produtos() -> ProdutosResponse {
        let pool = get_pool();
        get_all(&pool).unwrap()
    }

    pub fn get_first_produtos_id() -> Uuid {
        get_all_produtos().0[0].id
    }

    #[actix_rt::test]
    async fn it_gets_a_produto() {
        let first_produto = &get_all_produtos().0[0];
        let produto_id: Path<Uuid> = get_first_produtos_id().into();
        let response = get_produto(produto_id, get_data_pool()).await.unwrap();
        assert_eq!(response.into_inner(), *first_produto);
    }

    #[actix_rt::test]
    async fn it_doesnt_find_a_produto() {
        let uuid = Uuid::new_v4();
        let produto_id: Path<Uuid> = uuid.into();
        let response = get_produto(produto_id, get_data_pool()).await;
        let expected_error = ApiError::NotFound(format!("Produto {} not found", uuid.to_string()));
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_error);
    }

    #[actix_rt::test]
    async fn it_gets_all_produtos() {
        let response = get_produtos(get_data_pool()).await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().into_inner().0[0], get_all_produtos().0[0]);
    }

    #[actix_rt::test]
    async fn it_creates_a_produto() {
        let params = Json(CreateProdutoRequest {
            descricao: "coca cola".into(),
            codigo_de_barras: Some("123456789".into()),
            icms: Some("18".into()),
            unidade_medida: Some("un".into()),
            preco_venda: None,
            preco_minimo: None,
            custo_medio: None,
            pis: Some("1.65".into()),
            cofins: Some("7.60".into()),
            ncm: Some("22021000".into()),
            cest: Some("1705600".into()),
            cod_ipi: Some("999".into()),
            porcentagem_ipi: None,
        });
        let response = create_produto(get_data_pool(), Json(params.clone()))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(response.descricao, params.descricao);
        delete(&get_data_pool(), response.id);
    }

    #[actix_rt::test]
    async fn it_updates_a_produto() {
        let first_produto = model_create_produto_by_email("teste_handler_update@teste.com").unwrap();
        let produto_id: Path<Uuid> = Path::from(first_produto.id);
        let params = Json(UpdateProdutoRequest {
            descricao: "coca cola".into(),
            codigo_de_barras: Some("123456789".into()),
            icms: Some("18".into()),
            unidade_medida: Some("un".into()),
            preco_venda: None,
            preco_minimo: None,
            custo_medio: None,
            pis: Some("1.65".into()),
            cofins: Some("7.60".into()),
            ncm: Some("22021000".into()),
            cest: Some("1705600".into()),
            cod_ipi: Some("999".into()),
            porcentagem_ipi: None,
        });
        let response = update_produto(produto_id, get_data_pool(), Json(params.clone()))
            .await
            .unwrap();
        assert_eq!(response.into_inner().descricao, params.descricao);
        delete(&get_data_pool(), first_produto.id);
    }

    #[actix_rt::test]
    async fn it_deletes_a_produto() {
        let created = model_create_produto_by_email("teste_handler_delete@teste.com");
        let produto_id = created.unwrap().id;
        let produto_id_path: Path<Uuid> = produto_id.into();
        let produto = find(&get_pool(), produto_id);
        assert!(produto.is_ok());
        delete_produto(produto_id_path, get_data_pool()).await.unwrap();
        let produto = find(&get_pool(), produto_id);
        assert!(produto.is_err());
    }
}
