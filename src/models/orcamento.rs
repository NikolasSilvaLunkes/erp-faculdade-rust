use crate::auth::hash;
use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::orcamento::{OrcamentoResponse, OrcamentosResponse};
use crate::models::orcamento_produto::OrcamentoProduto;
use crate::schema::orcamentos;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Utc};
use diesel::prelude::*;
use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct Orcamento {
    pub id: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewOrcamento {
    pub id: String,
    pub created_by: String,
}

// #[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
// #[table_name = "orcamentos"]
// pub struct UpdateOrcamento {
//     pub id: String,
//     pub descricao: String,
//     pub codigo_de_barras: Option<String>,
//     pub icms: Option<String>,
//     pub unidade_medida: Option<String>,
//     pub preco_venda: Option<BigDecimal>,
//     pub preco_minimo: Option<BigDecimal>,
//     pub custo_medio: Option<BigDecimal>,
//     pub pis: Option<String>,
//     pub cofins: Option<String>,
//     pub ncm: Option<String>,
//     pub cest: Option<String>,
//     pub cod_ipi: Option<String>,
//     pub porcentagem_ipi: Option<BigDecimal>,
//     pub updated_by: String,
// }

/// Get all orcamentos
pub fn get_all(pool: &PoolType) -> Result<OrcamentosResponse, ApiError> {
    use crate::schema::orcamentos::dsl::orcamentos;

    let mut conn = pool.get()?;
    let all_orcamentos = orcamentos.load(&mut conn)?;

    Ok(all_orcamentos.into())
}

/// Find a orcamento by the orcamento's id or error out
pub fn find(pool: &PoolType, orcamento_id: Uuid) -> Result<OrcamentoResponse, ApiError> {
    use crate::schema::orcamentos::dsl::{id, orcamentos};

    let not_found = format!("Orcamento {} not found", orcamento_id);
    let mut conn = pool.get()?;
    let orcamento = orcamentos
        .filter(id.eq(orcamento_id.to_string()))
        .first::<Orcamento>(&mut conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(orcamento.into())
}

/// Create a new orcamento
pub fn create(pool: &PoolType, new_orcamento: &Orcamento) -> Result<OrcamentoResponse, ApiError> {
    use crate::schema::orcamentos::dsl::orcamentos;

    let mut conn = pool.get()?;
    diesel::insert_into(orcamentos).values(new_orcamento).execute(&mut conn)?;
    Ok(new_orcamento.clone().into())
}

/// Update a orcamento
// pub fn update(pool: &PoolType, update_orcamento: &UpdateOrcamento) -> Result<OrcamentoResponse, ApiError> {
//     use crate::schema::orcamentos::dsl::{id, orcamentos};

//     let mut conn = pool.get()?;
//     diesel::update(orcamentos)
//         .filter(id.eq(update_orcamento.id.clone()))
//         .set(update_orcamento)
//         .execute(&mut conn)?;
//     find(&pool, Uuid::parse_str(&update_orcamento.id)?)
// }

/// Delete a orcamento
pub fn delete(pool: &PoolType, orcamento_id: Uuid) -> Result<(), ApiError> {
    use crate::schema::orcamentos::dsl::{id, orcamentos};

    let mut conn = pool.get()?;
    diesel::delete(orcamentos)
        .filter(id.eq(orcamento_id.to_string()))
        .execute(&mut conn)?;
    Ok(())
}

impl From<NewOrcamento> for Orcamento {
    fn from(orcamento: NewOrcamento) -> Self {
        Orcamento {
            id: orcamento.id,
            created_by: orcamento.created_by,
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

    pub fn get_all_orcamentos() -> Result<OrcamentosResponse, ApiError> {
        let pool = get_pool();
        get_all(&pool)
    }

    pub fn create_orcamento() -> Result<OrcamentoResponse, ApiError> {
        let orcamento_id = Uuid::new_v4();
        let new_orcamento = NewOrcamento {
            id: orcamento_id.to_string(),
            created_by: orcamento_id.to_string(),
        };
        let orcamento: Orcamento = new_orcamento.into();
        create(&get_pool(), &orcamento)
    }

    pub fn create_orcamento_by_email(email: &str) -> Result<OrcamentoResponse, ApiError> {
        let orcamento_id = Uuid::new_v4();
        let new_orcamento = NewOrcamento {
            id: orcamento_id.to_string(),
            created_by: orcamento_id.to_string(),
        };
        let orcamento: Orcamento = new_orcamento.into();
        create(&get_pool(), &orcamento)
    }

    #[test]
    fn it_gets_a_orcamento() {
        let orcamentos = get_all_orcamentos();
        assert!(orcamentos.is_ok());
    }

    #[test]
    fn test_find() {
        let orcamentos = get_all_orcamentos().unwrap();
        let orcamento = &orcamentos.0[0];
        let found_orcamento = find(&get_pool(), orcamento.id).unwrap();
        assert_eq!(orcamento, &found_orcamento);
    }

    #[test]
    fn it_doesnt_find_a_orcamento() {
        let orcamento_id = Uuid::new_v4();
        let not_found_orcamento = find(&get_pool(), orcamento_id);
        assert!(not_found_orcamento.is_err());
    }

    #[test]
    fn it_creates_a_orcamento() {
        let created = create_orcamento();
        assert!(created.is_ok());
        let unwrapped = created.unwrap();
        let found_orcamento = find(&get_pool(), unwrapped.id.clone()).unwrap();
        let orcamento_id = unwrapped.id;
        delete(&get_pool(), orcamento_id).unwrap();
        assert_eq!(unwrapped, found_orcamento);
    }

    // #[test]
    // fn it_updates_a_orcamento() {
    //     let created = create_orcamento_by_email("teste_model_update6@teste.com").unwrap();
    //     let update_orcamento = UpdateOrcamento {
    //         id: created.id.to_string(),
    //         descricao: "Model".to_string(),
    //         codigo_de_barras: Some("1".to_string()),
    //         icms: Some("1".to_string()),
    //         unidade_medida: Some("UN".to_string()),
    //         preco_venda: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
    //         preco_minimo: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
    //         custo_medio: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
    //         pis: Some("1".to_string()),
    //         cofins: Some("00".to_string()),
    //         ncm: Some("00".to_string()),
    //         cest: Some("00".to_string()),
    //         cod_ipi: Some("00".to_string()),
    //         porcentagem_ipi: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
    //         updated_by: created.id.to_string(),
    //     };
    //     let updated = update(&get_pool(), &update_orcamento);
    //     assert!(updated.is_ok());
    //     let found_orcamento = find(&get_pool(), created.id).unwrap();
    //     assert_eq!(updated.unwrap(), found_orcamento);
    //     delete(&get_pool(), created.id);
    // }

    // #[test]
    // fn it_fails_to_update_a_nonexistent_orcamento() {
    //     let orcamento_id = Uuid::new_v4();
    //     let update_orcamento = UpdateOrcamento {
    //         id: orcamento_id.to_string(),
    //         descricao: "Model".to_string(),
    //         codigo_de_barras: Some("12345678901".to_string()),
    //         icms: Some("123456789".to_string()),
    //         unidade_medida: Some("123456789".to_string()),
    //         preco_venda: Some(BigDecimal::from_str("1.00").unwrap()),
    //         preco_minimo: Some(BigDecimal::from_str("1.00").unwrap()),
    //         custo_medio: Some(BigDecimal::from_str("1.00").unwrap().with_scale(2)),
    //         pis: Some("123456789".to_string()),
    //         cofins: Some("00".to_string()),
    //         ncm: Some("00".to_string()),
    //         cest: Some("00".to_string()),
    //         cod_ipi: Some("00".to_string()),
    //         porcentagem_ipi: Some(BigDecimal::from_str("1.00").unwrap()),
    //         updated_by: orcamento_id.to_string(),
    //     };
    //     let updated = update(&get_pool(), &update_orcamento);
    //     assert!(updated.is_err());
    // }

    #[test]
    fn it_deletes_a_orcamento() {
        let created = create_orcamento_by_email("teste_model_delete@teste.com");
        let orcamento_id = created.unwrap().id;
        let orcamento = find(&get_pool(), orcamento_id);
        assert!(orcamento.is_ok());
        delete(&get_pool(), orcamento_id).unwrap();
        let orcamento = find(&get_pool(), orcamento_id);
        assert!(orcamento.is_err());
    }
}
