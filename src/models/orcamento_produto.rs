use crate::auth::hash;
use crate::database::PoolType;
use crate::errors::ApiError;
//use crate::handlers::orcamento_produto::{OrcamentoProdutoResponse, OrcamentoProdutosResponse};
use crate::schema::orcamento_produtos;
use crate::handlers::orcamento_produto::{OrcamentoProdutoResponse};
use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Utc};
use diesel::prelude::*;
use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct OrcamentoProduto {
    pub id: String,
    pub id_produto: String,
    pub id_orcamento: String,
    pub quantidade: i32,
    pub created_by: String,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewOrcamentoProduto {
    pub id: String,
    pub id_produto: String,
    pub id_orcamento: String,
    pub quantidade: i32,
    pub created_by: String,
}

/// Find a orcamento_produto by the orcamento_produto's id or error out
pub fn find(pool: &PoolType, orcamento_produto_id: Uuid) -> Result<OrcamentoProdutoResponse, ApiError> {
    use crate::schema::orcamento_produtos::dsl::{id, orcamento_produtos};

    let not_found = format!("OrcamentoProduto {} not found", orcamento_produto_id);
    let mut conn = pool.get()?;
    let orcamento_produto = orcamento_produtos
        .filter(id.eq(orcamento_produto_id.to_string()))
        .first::<OrcamentoProduto>(&mut conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(orcamento_produto.into())
}

/// Create a new orcamento_produto
pub fn create(pool: &PoolType, new_orcamento_produto: &OrcamentoProduto) -> Result<OrcamentoProdutoResponse, ApiError> {
    use crate::schema::orcamento_produtos::dsl::orcamento_produtos;

    let mut conn = pool.get()?;
    diesel::insert_into(orcamento_produtos).values(new_orcamento_produto).execute(&mut conn)?;
    Ok(new_orcamento_produto.clone().into())
}

/// Delete a orcamento_produto
pub fn delete(pool: &PoolType, orcamento_produto_id: Uuid) -> Result<(), ApiError> {
    use crate::schema::orcamento_produtos::dsl::{id, orcamento_produtos};

    let mut conn = pool.get()?;
    diesel::delete(orcamento_produtos)
        .filter(id.eq(orcamento_produto_id.to_string()))
        .execute(&mut conn)?;
    Ok(())
}

impl From<NewOrcamentoProduto> for OrcamentoProduto {
    fn from(orcamento_produto: NewOrcamentoProduto) -> Self {
        OrcamentoProduto {
            id: orcamento_produto.id,
            id_produto: orcamento_produto.id_produto,
            id_orcamento: orcamento_produto.id_orcamento,
            quantidade: orcamento_produto.quantidade,
            created_by: orcamento_produto.created_by,
            created_at: Utc::now().naive_utc(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use bigdecimal::FromPrimitive;

    use super::*;
    use crate::tests::helpers::tests::get_pool;

    pub fn create_orcamento_produto() -> Result<OrcamentoProdutoResponse, ApiError> {
        let orcamento_produto_id = Uuid::new_v4();
        let new_orcamento_produto = NewOrcamentoProduto {
            id: orcamento_produto_id.to_string(),
            id_produto: "00000000-0000-0000-0000-000000000000".to_string(),
            id_orcamento: "00000000-0000-0000-0000-000000000000".to_string(),
            quantidade: 1,
            created_by: orcamento_produto_id.to_string(),
        };
        let orcamento_produto: OrcamentoProduto = new_orcamento_produto.into();
        create(&get_pool(), &orcamento_produto)
    }

    #[test]
    fn it_doesnt_find_a_orcamento_produto() {
        let orcamento_produto_id = Uuid::new_v4();
        let not_found_orcamento_produto = find(&get_pool(), orcamento_produto_id);
        assert!(not_found_orcamento_produto.is_err());
    }

    #[test]
    fn it_creates_a_orcamento_produto() {
        let created = create_orcamento_produto();
        assert!(created.is_ok());
        let unwrapped = created.unwrap();
        let found_orcamento_produto = find(&get_pool(), unwrapped.id.clone()).unwrap();
        let orcamento_produto_id = unwrapped.id;
        delete(&get_pool(), orcamento_produto_id).unwrap();
        assert_eq!(unwrapped, found_orcamento_produto);
    }

    #[test]
    fn it_deletes_a_orcamento_produto() {
        let created = create_orcamento_produto();
        let orcamento_produto_id = created.unwrap().id;
        let orcamento_produto = find(&get_pool(), orcamento_produto_id);
        assert!(orcamento_produto.is_ok());
        delete(&get_pool(), orcamento_produto_id).unwrap();
        let orcamento_produto = find(&get_pool(), orcamento_produto_id);
        assert!(orcamento_produto.is_err());
    }
}
