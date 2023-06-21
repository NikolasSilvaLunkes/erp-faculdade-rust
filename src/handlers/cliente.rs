use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::cliente::{create, delete, find, get_all, update, NewCliente, UpdateCliente, Cliente};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClienteResponse {
    pub id: Uuid,
    pub nome: String,
    pub sobrenome: String,
    pub cpf: Option<String>,
    pub rg: Option<String>,
    pub data_nascimento: Option<NaiveDateTime>,
    pub sexo: Option<String>,
    pub estado_civil: Option<String>,
    pub telefone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClientesResponse(pub Vec<ClienteResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateClienteRequest {
    #[validate(length(
        min = 3,
        message = "O primeiro nome deve ter pelo menos 3 caracteres"
    ))]
    pub nome: String,

    #[validate(length(
        min = 3,
        message = "O sobrenome deve ter pelo menos 3 caracteres"
    ))]
    pub sobrenome: String,

    #[validate(length(
        min = 11, max = 11,
        message = "O cpf deve ter 11 caracteres"
    ))]
    pub cpf: Option<String>,

    #[validate(length(
        min = 9, max = 9,
        message = "O rg deve ter 9 caracteres"
    ))]
    pub rg: Option<String>,

    #[validate(length(
        min = 1, max = 1,
        message = "O sexo deve ter 1 caracteres"
    ))]
    pub sexo: Option<String>,

    #[validate(length(
        min = 1,
        message = "O estado civil deve ter ao menos um caracter"
    ))]
    pub estado_civil: Option<String>,

    #[validate(length(
        min = 9, max = 11,
        message = "O telefone deve ter de 9 a 11 caracteres"
    ))]
    pub telefone: Option<String>,

    pub data_nascimento: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateClienteRequest {
    #[validate(length(
        min = 3,
        message = "O primeiro nome deve ter pelo menos 3 caracteres"
    ))]
    pub nome: String,

    #[validate(length(
        min = 3,
        message = "O sobrenome deve ter pelo menos 3 caracteres"
    ))]
    pub sobrenome: String,

    #[validate(length(
        min = 11, max = 11,
        message = "O cpf deve ter 11 caracteres"
    ))]
    pub cpf: Option<String>,

    #[validate(length(
        min = 9, max = 9,
        message = "O rg deve ter 9 caracteres"
    ))]
    pub rg: Option<String>,

    #[validate(length(
        min = 1, max = 1,
        message = "O sexo deve ter 1 caracteres"
    ))]
    pub sexo: Option<String>,

    #[validate(length(
        min = 1,
        message = "O estado civil deve ter no minimo 1 caracter"
    ))]
    pub estado_civil: Option<String>,

    #[validate(length(
        min = 9, max = 11,
        message = "O telefone deve ter de 9 a 11 caracteres"
    ))]
    pub telefone: Option<String>,

    pub data_nascimento: Option<NaiveDateTime>,
}

/// Get a cliente
pub async fn get_cliente(
    cliente_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<Json<ClienteResponse>, ApiError> {
    let cliente = block(move || find(&pool, *cliente_id)).await?;
    respond_json(cliente)
}

/// Get all clientes
pub async fn get_clientes(pool: Data<PoolType>) -> Result<Json<ClientesResponse>, ApiError> {
    let clientes = block(move || get_all(&pool)).await?;
    respond_json(clientes)
}

/// Create a cliente
pub async fn create_cliente(
    pool: Data<PoolType>,
    params: Json<CreateClienteRequest>,
) -> Result<Json<ClienteResponse>, ApiError> {
    validate(&params)?;

    // temporarily use the new cliente's id for created_at/updated_at
    // update when auth is added
    let cliente_id = Uuid::new_v4();
    let new_cliente: Cliente = NewCliente {
        id: cliente_id.to_string(),
        nome: params.nome.to_string(),
        sobrenome: params.sobrenome.to_string(),
        cpf: params.cpf.clone(),
        rg: params.rg.clone(),
        data_nascimento: params.data_nascimento.clone(),
        sexo: params.sexo.clone(),
        estado_civil: params.estado_civil.clone(),
        telefone: params.telefone.clone(),
        created_by: cliente_id.to_string(),
        updated_by: cliente_id.to_string(),
    }
    .into();
    let cliente = block(move || create(&pool, &new_cliente)).await?;
    respond_json(cliente.into())
}

/// Update a cliente
pub async fn update_cliente(
    cliente_id: Path<Uuid>,
    pool: Data<PoolType>,
    params: Json<UpdateClienteRequest>,
) -> Result<Json<ClienteResponse>, ApiError> {
    validate(&params)?;

    // temporarily use the cliente's id for updated_at
    // update when auth is added
    let update_cliente = UpdateCliente {
        id: cliente_id.to_string(),
        nome: params.nome.to_string(),
        sobrenome: params.sobrenome.to_string(),
        cpf: params.cpf.clone(),
        rg: params.rg.clone(),
        data_nascimento: params.data_nascimento.clone(),
        sexo: params.sexo.clone(),
        estado_civil: params.estado_civil.clone(),
        telefone:params.telefone.clone(),
        updated_by: cliente_id.to_string(),
    };
    let cliente = block(move || update(&pool, &update_cliente)).await?;
    respond_json(cliente.into())
}

/// Delete a cliente
pub async fn delete_cliente(
    cliente_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<HttpResponse, ApiError> {
    block(move || delete(&pool, *cliente_id)).await?;
    respond_ok()
}

impl From<Cliente> for ClienteResponse {
    fn from(cliente: Cliente) -> Self {
        ClienteResponse {
            id: Uuid::parse_str(&cliente.id).unwrap(),
            nome: cliente.nome.to_string(),
            sobrenome: cliente.sobrenome.to_string(),
            cpf: cliente.cpf,
            rg: cliente.rg,
            data_nascimento: cliente.data_nascimento,
            sexo: cliente.sexo,
            estado_civil: cliente.estado_civil,
            telefone: cliente.telefone,
        }
    }
}

impl From<Vec<Cliente>> for ClientesResponse {
    fn from(clientes: Vec<Cliente>) -> Self {
        ClientesResponse(clientes.into_par_iter().map(|cliente| cliente.into()).collect())
    }
}

///Testes
///Testes
///Testes
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::models::cliente::tests::create_cliente as model_create_cliente;
    use crate::tests::helpers::tests::{get_data_pool, get_pool};

    pub fn get_all_clientes() -> ClientesResponse {
        let pool = get_pool();
        get_all(&pool).unwrap()
    }

    pub fn get_first_clientes_id() -> Uuid {
        get_all_clientes().0[0].id
    }

    #[actix_rt::test]
    async fn it_gets_a_cliente() {
        let first_cliente = &get_all_clientes().0[0];
        let cliente_id: Path<Uuid> = get_first_clientes_id().into();
        let response = get_cliente(cliente_id, get_data_pool()).await.unwrap();
        assert_eq!(response.into_inner(), *first_cliente);
    }

    #[actix_rt::test]
    async fn it_doesnt_find_a_cliente() {
        let uuid = Uuid::new_v4();
        let cliente_id: Path<Uuid> = uuid.into();
        let response = get_cliente(cliente_id, get_data_pool()).await;
        let expected_error = ApiError::NotFound(format!("Cliente {} not found", uuid.to_string()));
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_error);
    }

    #[actix_rt::test]
    async fn it_gets_all_clientes() {
        let response = get_clientes(get_data_pool()).await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().into_inner().0[0], get_all_clientes().0[0]);
    }

    #[actix_rt::test]
    async fn it_creates_a_cliente() {
        let params = Json(CreateClienteRequest {
            nome: "Satoshi".into(),
            sobrenome: "Nakamoto".into(),
            cpf: Some("12345678901".into()),
            rg: Some("123456789".into()),
            data_nascimento: Some(NaiveDate::from_ymd(1990, 1, 1).and_hms(0, 0, 0)),
            sexo: Some("M".into()),
            estado_civil: Some("Solteiro".into()),
            telefone: Some("1234567890".into()),
        });
        let response = create_cliente(get_data_pool(), Json(params.clone()))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(response.nome, params.nome);
        delete(&get_data_pool(), response.id);
    }

    #[actix_rt::test]
    async fn it_updates_a_cliente() {
        let first_cliente = model_create_cliente().unwrap();
        let cliente_id: Path<Uuid> = Path::from(first_cliente.id);
        let params = Json(UpdateClienteRequest {
            nome: first_cliente.nome.clone(),
            sobrenome: first_cliente.sobrenome.clone(),
            cpf: first_cliente.cpf.clone(),
            rg: first_cliente.rg.clone(),
            data_nascimento: first_cliente.data_nascimento.clone(),
            sexo: first_cliente.sexo.clone(),
            estado_civil: first_cliente.estado_civil.clone(),
            telefone: first_cliente.telefone.clone(),
        });
        let response = update_cliente(cliente_id, get_data_pool(), Json(params.clone()))
            .await
            .unwrap();
        assert_eq!(response.into_inner().nome, params.nome);
        delete(&get_data_pool(), first_cliente.id);
    }

    #[actix_rt::test]
    async fn it_deletes_a_cliente() {
        let created = model_create_cliente();
        let cliente_id = created.unwrap().id;
        let cliente_id_path: Path<Uuid> = cliente_id.into();
        let cliente = find(&get_pool(), cliente_id);
        assert!(cliente.is_ok());
        delete_cliente(cliente_id_path, get_data_pool()).await.unwrap();
        let cliente = find(&get_pool(), cliente_id);
        assert!(cliente.is_err());
    }
}
