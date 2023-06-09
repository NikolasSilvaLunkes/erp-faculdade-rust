use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::user::{create, delete, find, get_all, update, NewUser, UpdateUser, User};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserResponse {
    pub id: Uuid,
    pub nome: String,
    pub sobrenome: String,
    pub cpf: Option<String>,
    pub rg: Option<String>,
    pub data_nascimento: Option<NaiveDateTime>,
    pub sexo: Option<String>,
    pub estado_civil: Option<String>,
    pub telefone: Option<String>,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UsersResponse(pub Vec<UserResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateUserRequest {
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

    #[validate(email(message = "O email deve ser valido"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "A senha deve deve ter pelo menos 6 caracteres"
    ))]
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUserRequest {
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

    #[validate(email(message = "O email deve ser valido"))]
    pub email: String,
}

/// Get a user
pub async fn get_user(
    user_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = block(move || find(&pool, *user_id)).await?;
    respond_json(user)
}

/// Get all users
pub async fn get_users(pool: Data<PoolType>) -> Result<Json<UsersResponse>, ApiError> {
    let users = block(move || get_all(&pool)).await?;
    respond_json(users)
}

/// Create a user
pub async fn create_user(
    pool: Data<PoolType>,
    params: Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    validate(&params)?;

    // temporarily use the new user's id for created_at/updated_at
    // update when auth is added
    let user_id = Uuid::new_v4();
    let new_user: User = NewUser {
        id: user_id.to_string(),
        nome: params.nome.to_string(),
        sobrenome: params.sobrenome.to_string(),
        cpf: params.cpf.clone(),
        rg: params.rg.clone(),
        data_nascimento: params.data_nascimento.clone(),
        sexo: params.sexo.clone(),
        estado_civil: params.estado_civil.clone(),
        telefone: params.telefone.clone(),
        email: params.email.to_string(),
        password: params.password.to_string(),
        created_by: user_id.to_string(),
        updated_by: user_id.to_string(),
    }
    .into();
    let user = block(move || create(&pool, &new_user)).await?;
    respond_json(user.into())
}

/// Update a user
pub async fn update_user(
    user_id: Path<Uuid>,
    pool: Data<PoolType>,
    params: Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    validate(&params)?;

    // temporarily use the user's id for updated_at
    // update when auth is added
    let update_user = UpdateUser {
        id: user_id.to_string(),
        nome: params.nome.to_string(),
        sobrenome: params.sobrenome.to_string(),
        cpf: params.cpf.clone(),
        rg: params.rg.clone(),
        data_nascimento: params.data_nascimento.clone(),
        sexo: params.sexo.clone(),
        estado_civil: params.estado_civil.clone(),
        telefone:params.telefone.clone(),
        email: params.email.to_string(),
        updated_by: user_id.to_string(),
    };
    let user = block(move || update(&pool, &update_user)).await?;
    respond_json(user.into())
}

/// Delete a user
pub async fn delete_user(
    user_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<HttpResponse, ApiError> {
    block(move || delete(&pool, *user_id)).await?;
    respond_ok()
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: Uuid::parse_str(&user.id).unwrap(),
            nome: user.nome.to_string(),
            sobrenome: user.sobrenome.to_string(),
            cpf: user.cpf,
            rg: user.rg,
            data_nascimento: user.data_nascimento,
            sexo: user.sexo,
            estado_civil: user.estado_civil,
            telefone: user.telefone,
            email: user.email.to_string(),
        }
    }
}

impl From<Vec<User>> for UsersResponse {
    fn from(users: Vec<User>) -> Self {
        UsersResponse(users.into_par_iter().map(|user| user.into()).collect())
    }
}

///Testes
///Testes
///Testes
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::models::user::tests::create_user as model_create_user;
    use crate::models::user::tests::create_user_by_email as model_create_user_by_email;
    use crate::models::user::delete as model_delete;
    use crate::tests::helpers::tests::{get_data_pool, get_pool};

    pub fn get_all_users() -> UsersResponse {
        let pool = get_pool();
        get_all(&pool).unwrap()
    }

    pub fn get_first_users_id() -> Uuid {
        get_all_users().0[0].id
    }

    #[actix_rt::test]
    async fn it_gets_a_user() {
        let first_user = &get_all_users().0[0];
        let user_id: Path<Uuid> = get_first_users_id().into();
        let response = get_user(user_id, get_data_pool()).await.unwrap();
        assert_eq!(response.into_inner(), *first_user);
    }

    #[actix_rt::test]
    async fn it_doesnt_find_a_user() {
        let uuid = Uuid::new_v4();
        let user_id: Path<Uuid> = uuid.into();
        let response = get_user(user_id, get_data_pool()).await;
        let expected_error = ApiError::NotFound(format!("User {} not found", uuid.to_string()));
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_error);
    }

    #[actix_rt::test]
    async fn it_gets_all_users() {
        let response = get_users(get_data_pool()).await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().into_inner().0[0], get_all_users().0[0]);
    }

    #[actix_rt::test]
    async fn it_creates_a_user() {
        let params = Json(CreateUserRequest {
            nome: "Satoshi".into(),
            sobrenome: "Nakamoto".into(),
            cpf: Some("12345678901".into()),
            rg: Some("123456789".into()),
            data_nascimento: Some(NaiveDate::from_ymd(1990, 1, 1).and_hms(0, 0, 0)),
            sexo: Some("M".into()),
            estado_civil: Some("Solteiro".into()),
            telefone: Some("1234567890".into()),
            email: "teste_handler_create@teste.com".into(),
            password: "123456".into(),
        });
        let response = create_user(get_data_pool(), Json(params.clone()))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(response.nome, params.nome);
        delete(&get_data_pool(), response.id);
    }

    #[actix_rt::test]
    async fn it_updates_a_user() {
        let first_user = model_create_user_by_email("teste_handler_update@teste.com").unwrap();
        let user_id: Path<Uuid> = Path::from(first_user.id);
        let params = Json(UpdateUserRequest {
            nome: first_user.nome.clone(),
            sobrenome: first_user.sobrenome.clone(),
            cpf: first_user.cpf.clone(),
            rg: first_user.rg.clone(),
            data_nascimento: first_user.data_nascimento.clone(),
            sexo: first_user.sexo.clone(),
            estado_civil: first_user.estado_civil.clone(),
            telefone: first_user.telefone.clone(),
            email: first_user.email.clone(),
        });
        let response = update_user(user_id, get_data_pool(), Json(params.clone()))
            .await
            .unwrap();
        assert_eq!(response.into_inner().nome, params.nome);
        delete(&get_data_pool(), first_user.id);
    }

    #[actix_rt::test]
    async fn it_deletes_a_user() {
        let created = model_create_user_by_email("teste_handler_delete@teste.com");
        let user_id = created.unwrap().id;
        let user_id_path: Path<Uuid> = user_id.into();
        let user = find(&get_pool(), user_id);
        assert!(user.is_ok());
        delete_user(user_id_path, get_data_pool()).await.unwrap();
        let user = find(&get_pool(), user_id);
        assert!(user.is_err());
    }
}
