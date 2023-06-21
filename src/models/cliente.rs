use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::cliente::{ClienteResponse, ClientesResponse};
use crate::schema::clientes;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct Cliente {
    pub id: String,
    pub nome: String,
    pub sobrenome: String,
    pub cpf: Option<String>,
    pub rg: Option<String>,
    pub data_nascimento: Option<NaiveDateTime>,
    pub sexo: Option<String>,
    pub estado_civil: Option<String>,
    pub telefone: Option<String>,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewCliente {
    pub id: String,
    pub nome: String,
    pub sobrenome: String,
    pub cpf: Option<String>,
    pub rg: Option<String>,
    pub data_nascimento: Option<NaiveDateTime>,
    pub sexo: Option<String>,
    pub estado_civil: Option<String>,
    pub telefone: Option<String>,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "clientes"]
pub struct UpdateCliente {
    pub id: String,
    pub nome: String,
    pub sobrenome: String,
    pub cpf: Option<String>,
    pub rg: Option<String>,
    pub data_nascimento: Option<NaiveDateTime>,
    pub sexo: Option<String>,
    pub estado_civil: Option<String>,
    pub telefone: Option<String>,
    pub updated_by: String,
}

/// Get all clientes
pub fn get_all(pool: &PoolType) -> Result<ClientesResponse, ApiError> {
    use crate::schema::clientes::dsl::clientes;

    let mut conn = pool.get()?;
    let all_clientes = clientes.load(&mut conn)?;

    Ok(all_clientes.into())
}

/// Find a cliente by the cliente's id or error out
pub fn find(pool: &PoolType, cliente_id: Uuid) -> Result<ClienteResponse, ApiError> {
    use crate::schema::clientes::dsl::{id, clientes};

    let not_found = format!("Cliente {} not found", cliente_id);
    let mut conn = pool.get()?;
    let cliente = clientes
        .filter(id.eq(cliente_id.to_string()))
        .first::<Cliente>(&mut conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(cliente.into())
}

/// Create a new cliente
pub fn create(pool: &PoolType, new_cliente: &Cliente) -> Result<ClienteResponse, ApiError> {
    use crate::schema::clientes::dsl::clientes;

    let mut conn = pool.get()?;
    diesel::insert_into(clientes).values(new_cliente).execute(&mut conn)?;
    Ok(new_cliente.clone().into())
}

/// Update a cliente
pub fn update(pool: &PoolType, update_cliente: &UpdateCliente) -> Result<ClienteResponse, ApiError> {
    use crate::schema::clientes::dsl::{id, clientes};

    let mut conn = pool.get()?;
    diesel::update(clientes)
        .filter(id.eq(update_cliente.id.clone()))
        .set(update_cliente)
        .execute(&mut conn)?;
    find(&pool, Uuid::parse_str(&update_cliente.id)?)
}

/// Delete a cliente
pub fn delete(pool: &PoolType, cliente_id: Uuid) -> Result<(), ApiError> {
    use crate::schema::clientes::dsl::{id, clientes};

    let mut conn = pool.get()?;
    diesel::delete(clientes)
        .filter(id.eq(cliente_id.to_string()))
        .execute(&mut conn)?;
    Ok(())
}

impl From<NewCliente> for Cliente {
    fn from(cliente: NewCliente) -> Self {
        Cliente {
            id: cliente.id,
            nome: cliente.nome,
            sobrenome: cliente.sobrenome,
            cpf: cliente.cpf,
            rg: cliente.rg,
            data_nascimento: cliente.data_nascimento,
            sexo: cliente.sexo,
            estado_civil: cliente.estado_civil,
            telefone: cliente.telefone,
            created_by: cliente.created_by,
            created_at: Utc::now().naive_utc(),
            updated_by: cliente.updated_by,
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::helpers::tests::get_pool;

    pub fn get_all_clientes() -> Result<ClientesResponse, ApiError> {
        let pool = get_pool();
        get_all(&pool)
    }

    pub fn create_cliente() -> Result<ClienteResponse, ApiError> {
        let cliente_id = Uuid::new_v4();
        let new_cliente = NewCliente {
            id: cliente_id.to_string(),
            nome: "Model".to_string(),
            sobrenome: "Test".to_string(),
            cpf: Some("12345678901".to_string()),
            rg: Some("123456789".to_string()),
            data_nascimento: Some(NaiveDateTime::new(NaiveDate::from_ymd(1990, 1, 1),NaiveTime::from_hms_milli(0, 0, 0, 0))),
            sexo: Some("M".to_string()),
            estado_civil: Some("Solteiro".to_string()),
            telefone: Some("12345678901".to_string()),
            created_by: cliente_id.to_string(),
            updated_by: cliente_id.to_string(),
        };
        let cliente: Cliente = new_cliente.into();
        create(&get_pool(), &cliente)
    }

    #[test]
    fn it_gets_a_cliente() {
        let clientes = get_all_clientes();
        assert!(clientes.is_ok());
    }

    #[test]
    fn test_find() {
        let clientes = get_all_clientes().unwrap();
        let cliente = &clientes.0[0];
        let found_cliente = find(&get_pool(), cliente.id).unwrap();
        assert_eq!(cliente, &found_cliente);
    }

    #[test]
    fn it_doesnt_find_a_cliente() {
        let cliente_id = Uuid::new_v4();
        let not_found_cliente = find(&get_pool(), cliente_id);
        assert!(not_found_cliente.is_err());
    }

    #[test]
    fn it_creates_a_cliente() {
        let created = create_cliente();
        assert!(created.is_ok());
        let unwrapped = created.unwrap();
        let found_cliente = find(&get_pool(), unwrapped.id.clone()).unwrap();
        let cliente_id = unwrapped.id;
        delete(&get_pool(), cliente_id).unwrap();
        assert_eq!(unwrapped, found_cliente);
    }

    #[test]
    fn it_updates_a_cliente() {
        let created = create_cliente().unwrap();
        let update_cliente = UpdateCliente {
            id: created.id.to_string(),
            nome: "ModelUpdate".to_string(),
            sobrenome: "TestUpdate".to_string(),
            cpf: Some("12345678901".to_string()),
            rg: Some("123456789".to_string()),
            data_nascimento: Some(NaiveDateTime::new(NaiveDate::from_ymd(1990, 1, 1),NaiveTime::from_hms_milli(0, 0, 0, 0))),
            sexo: Some("M".to_string()),
            estado_civil: Some("Solteiro".to_string()),
            telefone: Some("12345678901".to_string()),
            updated_by: created.id.to_string(),
        };
        let updated = update(&get_pool(), &update_cliente);
        assert!(updated.is_ok());
        let found_cliente = find(&get_pool(), created.id).unwrap();
        assert_eq!(updated.unwrap(), found_cliente);
        delete(&get_pool(), created.id);
    }

    #[test]
    fn it_fails_to_update_a_nonexistent_cliente() {
        let cliente_id = Uuid::new_v4();
        let update_cliente = UpdateCliente {
            id: cliente_id.to_string(),
            nome: "ModelUpdateFailure".to_string(),
            sobrenome: "TestUpdateFailure".to_string(),
            cpf: Some("12345678901".to_string()),
            rg: Some("123456789".to_string()),
            data_nascimento: Some(NaiveDateTime::new(NaiveDate::from_ymd(1990, 1, 1),NaiveTime::from_hms_milli(0, 0, 0, 0))),
            sexo: Some("M".to_string()),
            estado_civil: Some("Solteiro".to_string()),
            telefone: Some("12345678901".to_string()),
            updated_by: cliente_id.to_string(),
        };
        let updated = update(&get_pool(), &update_cliente);
        assert!(updated.is_err());
    }

    #[test]
    fn it_deletes_a_cliente() {
        let created = create_cliente();
        let cliente_id = created.unwrap().id;
        let cliente = find(&get_pool(), cliente_id);
        assert!(cliente.is_ok());
        delete(&get_pool(), cliente_id).unwrap();
        let cliente = find(&get_pool(), cliente_id);
        assert!(cliente.is_err());
    }
}
