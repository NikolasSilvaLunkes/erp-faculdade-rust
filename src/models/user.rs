use crate::auth::hash;
use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::user::{UserResponse, UsersResponse};
use crate::schema::users;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct User {
    pub id: String,
    pub nome: String,
    pub sobrenome: String,
    pub cpf: Option<String>,
    pub rg: Option<String>,
    pub data_nascimento: Option<NaiveDateTime>,
    pub sexo: Option<String>,
    pub estado_civil: Option<String>,
    pub telefone: Option<String>,
    pub email: String,
    pub password: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub id: String,
    pub nome: String,
    pub sobrenome: String,
    pub cpf: Option<String>,
    pub rg: Option<String>,
    pub data_nascimento: Option<NaiveDateTime>,
    pub sexo: Option<String>,
    pub estado_civil: Option<String>,
    pub telefone: Option<String>,
    pub email: String,
    pub password: String,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub id: String,
    pub nome: String,
    pub sobrenome: String,
    pub cpf: Option<String>,
    pub rg: Option<String>,
    pub data_nascimento: Option<NaiveDateTime>,
    pub sexo: Option<String>,
    pub estado_civil: Option<String>,
    pub telefone: Option<String>,
    pub email: String,
    pub updated_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
}

/// Get all users
pub fn get_all(pool: &PoolType) -> Result<UsersResponse, ApiError> {
    use crate::schema::users::dsl::users;

    let mut conn = pool.get()?;
    let all_users = users.load(&mut conn)?;

    Ok(all_users.into())
}

/// Find a user by the user's id or error out
pub fn find(pool: &PoolType, user_id: Uuid) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{id, users};

    let not_found = format!("User {} not found", user_id);
    let mut conn = pool.get()?;
    let user = users
        .filter(id.eq(user_id.to_string()))
        .first::<User>(&mut conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(user.into())
}

/// Find a user by the user's authentication information (email + password)
/// Return an Unauthorized error if it doesn't match
pub fn find_by_auth(
    pool: &PoolType,
    user_email: &str,
    user_password: &str,
) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{email, password, users};

    let mut conn = pool.get()?;
    let user = users
        .filter(email.eq(user_email.to_string()))
        .filter(password.eq(user_password.to_string()))
        .first::<User>(&mut conn)
        .map_err(|_| ApiError::Unauthorized("Invalid login".into()))?;
    Ok(user.into())
}

/// Create a new user
pub fn create(pool: &PoolType, new_user: &User) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::users;

    let mut conn = pool.get()?;
    diesel::insert_into(users).values(new_user).execute(&mut conn)?;
    Ok(new_user.clone().into())
}

/// Update a user
pub fn update(pool: &PoolType, update_user: &UpdateUser) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{id, users};

    let mut conn = pool.get()?;
    diesel::update(users)
        .filter(id.eq(update_user.id.clone()))
        .set(update_user)
        .execute(&mut conn)?;
    find(&pool, Uuid::parse_str(&update_user.id)?)
}

/// Delete a user
pub fn delete(pool: &PoolType, user_id: Uuid) -> Result<(), ApiError> {
    use crate::schema::users::dsl::{id, users};

    let mut conn = pool.get()?;
    diesel::delete(users)
        .filter(id.eq(user_id.to_string()))
        .execute(&mut conn)?;
    Ok(())
}

impl From<NewUser> for User {
    fn from(user: NewUser) -> Self {
        User {
            id: user.id,
            nome: user.nome,
            sobrenome: user.sobrenome,
            cpf: user.cpf,
            rg: user.rg,
            data_nascimento: user.data_nascimento,
            sexo: user.sexo,
            estado_civil: user.estado_civil,
            telefone: user.telefone,
            email: user.email,
            password: hash(&user.password),
            created_by: user.created_by,
            created_at: Utc::now().naive_utc(),
            updated_by: user.updated_by,
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::helpers::tests::get_pool;

    pub fn get_all_users() -> Result<UsersResponse, ApiError> {
        let pool = get_pool();
        get_all(&pool)
    }

    pub fn create_user() -> Result<UserResponse, ApiError> {
        let user_id = Uuid::new_v4();
        let new_user = NewUser {
            id: user_id.to_string(),
            nome: "Model".to_string(),
            sobrenome: "Test".to_string(),
            cpf: Some("12345678901".to_string()),
            rg: Some("123456789".to_string()),
            data_nascimento: Some(NaiveDateTime::new(NaiveDate::from_ymd(1990, 1, 1),NaiveTime::from_hms_milli(0, 0, 0, 0))),
            sexo: Some("M".to_string()),
            estado_civil: Some("Solteiro".to_string()),
            telefone: Some("12345678901".to_string()),
            email: "model_test@nothing.org".to_string(),
            password: "123456".to_string(),
            created_by: user_id.to_string(),
            updated_by: user_id.to_string(),
        };
        let user: User = new_user.into();
        create(&get_pool(), &user)
    }

    pub fn create_user_by_email(email: &str) -> Result<UserResponse, ApiError> {
        let user_id = Uuid::new_v4();
        let new_user = NewUser {
            id: user_id.to_string(),
            nome: "Model".to_string(),
            sobrenome: "Test".to_string(),
            cpf: Some("12345678901".to_string()),
            rg: Some("123456789".to_string()),
            data_nascimento: Some(NaiveDateTime::new(NaiveDate::from_ymd(1990, 1, 1),NaiveTime::from_hms_milli(0, 0, 0, 0))),
            sexo: Some("M".to_string()),
            estado_civil: Some("Solteiro".to_string()),
            telefone: Some("12345678901".to_string()),
            email: email.to_string(),
            password: "123456".to_string(),
            created_by: user_id.to_string(),
            updated_by: user_id.to_string(),
        };
        let user: User = new_user.into();
        create(&get_pool(), &user)
    }

    #[test]
    fn it_gets_a_user() {
        let users = get_all_users();
        assert!(users.is_ok());
    }

    #[test]
    fn test_find() {
        let users = get_all_users().unwrap();
        let user = &users.0[0];
        let found_user = find(&get_pool(), user.id).unwrap();
        assert_eq!(user, &found_user);
    }

    #[test]
    fn it_doesnt_find_a_user() {
        let user_id = Uuid::new_v4();
        let not_found_user = find(&get_pool(), user_id);
        assert!(not_found_user.is_err());
    }

    #[test]
    fn it_creates_a_user() {
        let created = create_user();
        assert!(created.is_ok());
        let unwrapped = created.unwrap();
        let found_user = find(&get_pool(), unwrapped.id.clone()).unwrap();
        let user_id = unwrapped.id;
        delete(&get_pool(), user_id).unwrap();
        assert_eq!(unwrapped, found_user);
    }

    #[test]
    fn it_updates_a_user() {
        let created = create_user_by_email("teste_model_update6@teste.com").unwrap();
        let update_user = UpdateUser {
            id: created.id.to_string(),
            nome: "ModelUpdate".to_string(),
            sobrenome: "TestUpdate".to_string(),
            cpf: Some("12345678901".to_string()),
            rg: Some("123456789".to_string()),
            data_nascimento: Some(NaiveDateTime::new(NaiveDate::from_ymd(1990, 1, 1),NaiveTime::from_hms_milli(0, 0, 0, 0))),
            sexo: Some("M".to_string()),
            estado_civil: Some("Solteiro".to_string()),
            telefone: Some("12345678901".to_string()),
            email: "teste_model_update6@teste.com".to_string(),
            updated_by: created.id.to_string(),
        };
        let updated = update(&get_pool(), &update_user);
        assert!(updated.is_ok());
        let found_user = find(&get_pool(), created.id).unwrap();
        assert_eq!(updated.unwrap(), found_user);
        delete(&get_pool(), created.id);
    }

    #[test]
    fn it_fails_to_update_a_nonexistent_user() {
        let user_id = Uuid::new_v4();
        let update_user = UpdateUser {
            id: user_id.to_string(),
            nome: "ModelUpdateFailure".to_string(),
            sobrenome: "TestUpdateFailure".to_string(),
            cpf: Some("12345678901".to_string()),
            rg: Some("123456789".to_string()),
            data_nascimento: Some(NaiveDateTime::new(NaiveDate::from_ymd(1990, 1, 1),NaiveTime::from_hms_milli(0, 0, 0, 0))),
            sexo: Some("M".to_string()),
            estado_civil: Some("Solteiro".to_string()),
            telefone: Some("12345678901".to_string()),
            email: "model-update-failure-test@nothing.org".to_string(),
            updated_by: user_id.to_string(),
        };
        let updated = update(&get_pool(), &update_user);
        assert!(updated.is_err());
    }

    #[test]
    fn it_deletes_a_user() {
        let created = create_user_by_email("teste_model_delete@teste.com");
        let user_id = created.unwrap().id;
        let user = find(&get_pool(), user_id);
        assert!(user.is_ok());
        delete(&get_pool(), user_id).unwrap();
        let user = find(&get_pool(), user_id);
        assert!(user.is_err());
    }
}
