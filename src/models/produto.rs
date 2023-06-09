use crate::auth::hash;
use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::produto::{ProdutoResponse, ProdutosResponse};
use crate::schema::produtos;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Utc};
use diesel::prelude::*;
use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct Produto {
    pub id: String,
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
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewProduto {
    pub id: String,
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
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "produtos"]
pub struct UpdateProduto {
    pub id: String,
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
    pub updated_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthProduto {
    pub id: String,
    pub email: String,
}

/// Get all produtos
pub fn get_all(pool: &PoolType) -> Result<ProdutosResponse, ApiError> {
    use crate::schema::produtos::dsl::produtos;

    let mut conn = pool.get()?;
    let all_produtos = produtos.load(&mut conn)?;

    Ok(all_produtos.into())
}

/// Find a produto by the produto's id or error out
pub fn find(pool: &PoolType, produto_id: Uuid) -> Result<ProdutoResponse, ApiError> {
    use crate::schema::produtos::dsl::{id, produtos};

    let not_found = format!("Produto {} not found", produto_id);
    let mut conn = pool.get()?;
    let produto = produtos
        .filter(id.eq(produto_id.to_string()))
        .first::<Produto>(&mut conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(produto.into())
}

/// Create a new produto
pub fn create(pool: &PoolType, new_produto: &Produto) -> Result<ProdutoResponse, ApiError> {
    use crate::schema::produtos::dsl::produtos;

    let mut conn = pool.get()?;
    diesel::insert_into(produtos).values(new_produto).execute(&mut conn)?;
    Ok(new_produto.clone().into())
}

/// Update a produto
pub fn update(pool: &PoolType, update_produto: &UpdateProduto) -> Result<ProdutoResponse, ApiError> {
    use crate::schema::produtos::dsl::{id, produtos};

    let mut conn = pool.get()?;
    diesel::update(produtos)
        .filter(id.eq(update_produto.id.clone()))
        .set(update_produto)
        .execute(&mut conn)?;
    find(&pool, Uuid::parse_str(&update_produto.id)?)
}

/// Delete a produto
pub fn delete(pool: &PoolType, produto_id: Uuid) -> Result<(), ApiError> {
    use crate::schema::produtos::dsl::{id, produtos};

    let mut conn = pool.get()?;
    diesel::delete(produtos)
        .filter(id.eq(produto_id.to_string()))
        .execute(&mut conn)?;
    Ok(())
}

impl From<NewProduto> for Produto {
    fn from(produto: NewProduto) -> Self {
        Produto {
            id: produto.id,
            descricao: produto.descricao,
            codigo_de_barras: produto.codigo_de_barras,
            icms: produto.icms,
            unidade_medida: produto.unidade_medida,
            preco_venda: produto.preco_venda,
            preco_minimo: produto.preco_minimo,
            custo_medio: produto.custo_medio,
            pis: produto.pis,
            cofins: produto.cofins,
            ncm: produto.ncm,
            cest: produto.cest,
            cod_ipi: produto.cod_ipi,
            porcentagem_ipi: produto.porcentagem_ipi,
            created_by: produto.created_by,
            created_at: Utc::now().naive_utc(),
            updated_by: produto.updated_by,
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use bigdecimal::FromPrimitive;

    use super::*;
    use crate::tests::helpers::tests::get_pool;

    pub fn get_all_produtos() -> Result<ProdutosResponse, ApiError> {
        let pool = get_pool();
        get_all(&pool)
    }

    pub fn create_produto() -> Result<ProdutoResponse, ApiError> {
        let produto_id = Uuid::new_v4();
        let new_produto = NewProduto {
            id: produto_id.to_string(),
            descricao: "Model".to_string(),
            codigo_de_barras: Some("1".to_string()),
            icms: Some("1".to_string()),
            unidade_medida: Some("UN".to_string()),
            preco_venda: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
            preco_minimo: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
            custo_medio: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
            pis: Some("1".to_string()),
            cofins: Some("00".to_string()),
            ncm: Some("00".to_string()),
            cest: Some("00".to_string()),
            cod_ipi: Some("00".to_string()),
            porcentagem_ipi: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
            created_by: produto_id.to_string(),
            updated_by: produto_id.to_string(),
        };
        let produto: Produto = new_produto.into();
        create(&get_pool(), &produto)
    }

    pub fn create_produto_by_email(email: &str) -> Result<ProdutoResponse, ApiError> {
        let produto_id = Uuid::new_v4();
        let new_produto = NewProduto {
            id: produto_id.to_string(),
            descricao: "Model".to_string(),
            codigo_de_barras: Some("1".to_string()),
            icms: Some("1".to_string()),
            unidade_medida: Some("UN".to_string()),
            preco_venda: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
            preco_minimo: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
            custo_medio: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
            pis: Some("1".to_string()),
            cofins: Some("00".to_string()),
            ncm: Some("00".to_string()),
            cest: Some("00".to_string()),
            cod_ipi: Some("00".to_string()),
            porcentagem_ipi: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
            created_by: produto_id.to_string(),
            updated_by: produto_id.to_string(),
        };
        let produto: Produto = new_produto.into();
        create(&get_pool(), &produto)
    }

    #[test]
    fn it_gets_a_produto() {
        let produtos = get_all_produtos();
        assert!(produtos.is_ok());
    }

    #[test]
    fn test_find() {
        let produtos = get_all_produtos().unwrap();
        let produto = &produtos.0[0];
        let found_produto = find(&get_pool(), produto.id).unwrap();
        assert_eq!(produto, &found_produto);
    }

    #[test]
    fn it_doesnt_find_a_produto() {
        let produto_id = Uuid::new_v4();
        let not_found_produto = find(&get_pool(), produto_id);
        assert!(not_found_produto.is_err());
    }

    #[test]
    fn it_creates_a_produto() {
        let created = create_produto();
        assert!(created.is_ok());
        let unwrapped = created.unwrap();
        let found_produto = find(&get_pool(), unwrapped.id.clone()).unwrap();
        let produto_id = unwrapped.id;
        delete(&get_pool(), produto_id).unwrap();
        assert_eq!(unwrapped, found_produto);
    }

    #[test]
    fn it_updates_a_produto() {
        let created = create_produto_by_email("teste_model_update6@teste.com").unwrap();
        let update_produto = UpdateProduto {
            id: created.id.to_string(),
            descricao: "Model".to_string(),
            codigo_de_barras: Some("1".to_string()),
            icms: Some("1".to_string()),
            unidade_medida: Some("UN".to_string()),
            preco_venda: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
            preco_minimo: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
            custo_medio: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
            pis: Some("1".to_string()),
            cofins: Some("00".to_string()),
            ncm: Some("00".to_string()),
            cest: Some("00".to_string()),
            cod_ipi: Some("00".to_string()),
            porcentagem_ipi: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
            updated_by: created.id.to_string(),
        };
        let updated = update(&get_pool(), &update_produto);
        assert!(updated.is_ok());
        let found_produto = find(&get_pool(), created.id).unwrap();
        assert_eq!(updated.unwrap(), found_produto);
        delete(&get_pool(), created.id);
    }

    #[test]
    fn it_fails_to_update_a_nonexistent_produto() {
        let produto_id = Uuid::new_v4();
        let update_produto = UpdateProduto {
            id: produto_id.to_string(),
            descricao: "Model".to_string(),
            codigo_de_barras: Some("12345678901".to_string()),
            icms: Some("123456789".to_string()),
            unidade_medida: Some("123456789".to_string()),
            preco_venda: Some(BigDecimal::from_str("1.00").unwrap()),
            preco_minimo: Some(BigDecimal::from_str("1.00").unwrap()),
            custo_medio: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
            pis: Some("123456789".to_string()),
            cofins: Some("00".to_string()),
            ncm: Some("00".to_string()),
            cest: Some("00".to_string()),
            cod_ipi: Some("00".to_string()),
            porcentagem_ipi: Some(BigDecimal::from_str("1.00").unwrap()),
            updated_by: produto_id.to_string(),
        };
        let updated = update(&get_pool(), &update_produto);
        assert!(updated.is_err());
    }

    #[test]
    fn it_deletes_a_produto() {
        let created = create_produto_by_email("teste_model_delete@teste.com");
        let produto_id = created.unwrap().id;
        let produto = find(&get_pool(), produto_id);
        assert!(produto.is_ok());
        delete(&get_pool(), produto_id).unwrap();
        let produto = find(&get_pool(), produto_id);
        assert!(produto.is_err());
    }
}
