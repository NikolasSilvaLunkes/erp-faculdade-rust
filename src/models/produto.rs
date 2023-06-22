use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::produto::{ProdutoResponse, ProdutosResponse};
use crate::schema::produto;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct Produto {
    pub id: String,
    pub descricao: String,
    pub codigo_de_barras: Option<String>,
    pub icms: Option<String>,
    pub unidade_medida: Option<String>,
    pub preco_venda: Option<f64>,
    pub preco_minimo: Option<f64>,
    pub custo_medio: Option<f64>,
    pub pis: Option<String>,
    pub cofins: Option<String>,
    pub ncm: Option<String>,
    pub cest: Option<String>,
    pub cod_ipi: Option<String>,
    pub porcentagem_ipi: Option<f64>,
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
    pub preco_venda: Option<f64>,
    pub preco_minimo: Option<f64>,
    pub custo_medio: Option<f64>,
    pub pis: Option<String>,
    pub cofins: Option<String>,
    pub ncm: Option<String>,
    pub cest: Option<String>,
    pub cod_ipi: Option<String>,
    pub porcentagem_ipi: Option<f64>,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "produto"]
pub struct UpdateProduto {
    pub id: String,
    pub descricao: String,
    pub codigo_de_barras: Option<String>,
    pub icms: Option<String>,
    pub unidade_medida: Option<String>,
    pub preco_venda: Option<f64>,
    pub preco_minimo: Option<f64>,
    pub custo_medio: Option<f64>,
    pub pis: Option<String>,
    pub cofins: Option<String>,
    pub ncm: Option<String>,
    pub cest: Option<String>,
    pub cod_ipi: Option<String>,
    pub porcentagem_ipi: Option<String>,
    pub updated_by: String,
}

/// Get all produto
pub fn get_all(pool: &PoolType) -> Result<ProdutosResponse, ApiError> {
    use crate::schema::produto::dsl::produto;

    let mut conn = pool.get()?;
    let all_produto = produto.load(&mut conn)?;

    Ok(all_produto.into())
}

/// Find a produto by the produto's id or error out
pub fn find(pool: &PoolType, produto_id: Uuid) -> Result<ProdutoResponse, ApiError> {
    use crate::schema::produto::dsl::{id, produto};

    let not_found = format!("Produto {} not found", produto_id);
    let mut conn = pool.get()?;
    let produto = produto
        .filter(id.eq(produto_id.to_string()))
        .first::<Produto>(&mut conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(produto.into())
}

/// Create a new produto
pub fn create(pool: &PoolType, new_produto: &Produto) -> Result<ProdutoResponse, ApiError> {
    use crate::schema::produto::dsl::produto;

    let mut conn = pool.get()?;
    diesel::insert_into(produto).values(new_produto).execute(&mut conn)?;
    Ok(new_produto.clone().into())
}

/// Update a produto
pub fn update(pool: &PoolType, update_produto: &UpdateProduto) -> Result<ProdutoResponse, ApiError> {
    use crate::schema::produto::dsl::{id, produto};

    let mut conn = pool.get()?;
    diesel::update(produto)
        .filter(id.eq(update_produto.id.clone()))
        .set(update_produto)
        .execute(&mut conn)?;
    find(&pool, Uuid::parse_str(&update_produto.id)?)
}

/// Delete a produto
pub fn delete(pool: &PoolType, produto_id: Uuid) -> Result<(), ApiError> {
    use crate::schema::produto::dsl::{id, produto};

    let mut conn = pool.get()?;
    diesel::delete(produto)
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
    use super::*;
    use crate::tests::helpers::tests::get_pool;

    pub fn get_all_produto() -> Result<ProdutosResponse, ApiError> {
        let pool = get_pool();
        get_all(&pool)
    }

    pub fn create_produto() -> Result<ProdutoResponse, ApiError> {
        let produto_id = Uuid::new_v4();
        let new_produto = NewProduto {
            id: produto_id.to_string(),
            descricao: "coca cola espumante".to_string(),
            codigo_de_barras: "124523324123565421435".to_string(),
            icms: "18".to_string(),
            unidade_medida: "01".to_string(),
            preco_venda: 7.99f64,
            preco_minimo: 6f64,
            custo_medio: 5f64,
            pis: "00".to_string(),
            cofins: "00".to_string(),
            ncm: "00".to_string(),
            cest: "00".to_string(),
            cod_ipi: "00".to_string(),
            porcentagem_ipi: 0f64,
            created_by: produto_id.to_string(),
            updated_by: produto_id.to_string(),
        };
        let produto: Produto = new_produto.into();
        create(&get_pool(), &produto)
    }

    #[test]
    fn it_gets_a_produto() {
        let produto = get_all_produto();
        assert!(produto.is_ok());
    }

    #[test]
    fn test_find() {
        let produto = get_all_produto().unwrap();
        let produto = &produto.0[0];
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
        let created = create_produto().unwrap();
        let update_produto = UpdateProduto {
            id: produto_id.to_string(),
            descricao: "coca cola espumante".to_string(),
            codigo_de_barras: "124523324123565421435".to_string(),
            icms: "18".to_string(),
            unidade_medida: "01".to_string(),
            preco_venda: 7.99f64,
            preco_minimo: 6f64,
            custo_medio: 5f64,
            pis: "00".to_string(),
            cofins: "00".to_string(),
            ncm: "00".to_string(),
            cest: "00".to_string(),
            cod_ipi: "00".to_string(),
            porcentagem_ipi: 0f64,
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
            descricao: "coca cola espumante".to_string(),
            codigo_de_barras: "124523324123565421435".to_string(),
            icms: "18".to_string(),
            unidade_medida: "01".to_string(),
            preco_venda: 7.99f64,
            preco_minimo: 6f64,
            custo_medio: 5f64,
            pis: "00".to_string(),
            cofins: "00".to_string(),
            ncm: "00".to_string(),
            cest: "00".to_string(),
            cod_ipi: "00".to_string(),
            porcentagem_ipi: 0f64,
            updated_by: produto_id.to_string(),
        };
        let updated = update(&get_pool(), &update_produto);
        assert!(updated.is_err());
    }

    #[test]
    fn it_deletes_a_produto() {
        let created = create_produto();
        let produto_id = created.unwrap().id;
        let produto = find(&get_pool(), produto_id);
        assert!(produto.is_ok());
        delete(&get_pool(), produto_id).unwrap();
        let produto = find(&get_pool(), produto_id);
        assert!(produto.is_err());
    }
}
