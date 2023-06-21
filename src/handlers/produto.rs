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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ProdutoResponse {
    pub id: String,
    pub descricao: String,
    pub codigo_de_barras: Option<String>,
    pub icms: Option<String>,
    pub unidade_medida: Option<String>,
    pub preco_venda: Option<f64>,
    pub preco_minimo: Option<f64>,
    pub preco_medio: Option<f64>,
    pub pis: Option<String>,
    pub cofins: Option<String>,
    pub ncm: Option<String>,
    pub cest: Option<String>,
    pub cod_ipi: Option<String>,
    pub porcentagem_ipi: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ProdutosResponse(pub Vec<ProdutoResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateProdutoRequest {
    #[validate(length(
        min = 6,
        message = "A descrião deve ter pelo menos 6 caracters"
    ))]
    pub codigo_de_barras: Option<String>,
    pub icms: Option<String>,
    pub unidade_medida: Option<String>,
    pub preco_venda: Option<f64>,
    pub preco_minimo: Option<f64>,
    pub preco_medio: Option<f64>,
    pub pis: Option<String>,
    pub cofins: Option<String>,
    pub ncm: Option<String>,
    pub cest: Option<String>,
    pub cod_ipi: Option<String>,
    pub porcentagem_ipi: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateProdutoRequest {
    #[validate(length(
        min = 6,
        message = "A descrião deve ter pelo menos 6 caracters"
    ))]
    pub codigo_de_barras: Option<String>,
    pub icms: Option<String>,
    pub unidade_medida: Option<String>,
    pub preco_venda: Option<f64>,
    pub preco_minimo: Option<f64>,
    pub preco_medio: Option<f64>,
    pub pis: Option<String>,
    pub cofins: Option<String>,
    pub ncm: Option<String>,
    pub cest: Option<String>,
    pub cod_ipi: Option<String>,
    pub porcentagem_ipi: Option<f64>,
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
    let produto = Uuid::new_v4();
    let new_produto: Produto = NewProduto {
        id: produto.to_string(),
        descricao: produto.descricao.to_string(),
        codigo_de_barras: produto.codigo_de_barras,
        icms: produto.icms,
        unidade_medida: produto.unidade_medida,
        preco_venda: produto.preco_venda,
        preco_minimo: produto.preco_minimo,
        preco_medio: produto.preco_medio,
        pis: produto.pis,
        cofins: produto.cofins,
        ncm: produto.ncm,
        cest: produto.cest,
        cod_ipi: produto.cod_ipi,
        porcentagem_ipi: produto.porcentagem_ipi,
        created_by: produto.to_string(),
        updated_by: produto.to_string(),
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
        nome: params.nome.to_string(),
        sobrenome: params.sobrenome.to_string(),
        cpf: params.cpf.clone(),
        rg: params.rg.clone(),
        data_nascimento: params.data_nascimento.clone(),
        sexo: params.sexo.clone(),
        estado_civil: params.estado_civil.clone(),
        telefone:params.telefone.clone(),
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
            nome: produto.nome.to_string(),
            sobrenome: produto.sobrenome.to_string(),
            cpf: produto.cpf,
            rg: produto.rg,
            data_nascimento: produto.data_nascimento,
            sexo: produto.sexo,
            estado_civil: produto.estado_civil,
            telefone: produto.telefone,
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
    use super::*;
    use crate::models::produto::tests::create_produto as model_create_produto;
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
            nome: "Satoshi".into(),
            sobrenome: "Nakamoto".into(),
            cpf: Some("12345678901".into()),
            rg: Some("123456789".into()),
            data_nascimento: Some(NaiveDate::from_ymd(1990, 1, 1).and_hms(0, 0, 0)),
            sexo: Some("M".into()),
            estado_civil: Some("Solteiro".into()),
            telefone: Some("1234567890".into()),
        });
        let response = create_produto(get_data_pool(), Json(params.clone()))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(response.nome, params.nome);
        delete(&get_data_pool(), response.id);
    }

    #[actix_rt::test]
    async fn it_updates_a_produto() {
        let first_produto = model_create_produto().unwrap();
        let produto_id: Path<Uuid> = Path::from(first_produto.id);
        let params = Json(UpdateProdutoRequest {
            nome: first_produto.nome.clone(),
            sobrenome: first_produto.sobrenome.clone(),
            cpf: first_produto.cpf.clone(),
            rg: first_produto.rg.clone(),
            data_nascimento: first_produto.data_nascimento.clone(),
            sexo: first_produto.sexo.clone(),
            estado_civil: first_produto.estado_civil.clone(),
            telefone: first_produto.telefone.clone(),
        });
        let response = update_produto(produto_id, get_data_pool(), Json(params.clone()))
            .await
            .unwrap();
        assert_eq!(response.into_inner().nome, params.nome);
        delete(&get_data_pool(), first_produto.id);
    }

    #[actix_rt::test]
    async fn it_deletes_a_produto() {
        let created = model_create_produto();
        let produto_id = created.unwrap().id;
        let produto_id_path: Path<Uuid> = produto_id.into();
        let produto = find(&get_pool(), produto_id);
        assert!(produto.is_ok());
        delete_produto(produto_id_path, get_data_pool()).await.unwrap();
        let produto = find(&get_pool(), produto_id);
        assert!(produto.is_err());
    }
}
