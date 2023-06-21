#[cfg(test)]
mod tests {
    use crate::handlers::user::{tests::get_first_users_id, CreateUserRequest};
    use crate::tests::helpers::tests::{assert_get, assert_post};
    use crate::models::user::delete as model_delete;
    use chrono::{NaiveDate};
    use actix_web::web::Path;
    use uuid::Uuid;

    const PATH: &str = "/api/v1/user";

    #[actix_rt::test]
    async fn it_gets_a_user() {
        let user_id: Path<Uuid> = get_first_users_id().into();
        let url = format!("{}/{}", PATH, user_id);
        assert_get(&url).await;
    }

    #[actix_rt::test]
    async fn it_gets_all_users() {
        assert_get(PATH).await;
    }

    // #[actix_rt::test]
    // async fn it_creates_a_user() {
    //     let params = CreateUserRequest {
    //         nome: "Satoshisan".into(),
    //         sobrenome: "Nakamoto".into(),
    //         cpf: Some("12345678901".into()),
    //         rg: Some("123456789".into()),
    //         data_nascimento: Some(NaiveDate::from_ymd(1990, 1, 1).and_hms(0, 0, 0)),
    //         sexo: Some("M".into()),
    //         estado_civil: Some("Solteiro".into()),
    //         telefone: Some("123456789".into()),
    //         email: "testingg@user.com".into(),
    //         password: "123456".into(),
    //     };
    //     let response = assert_post(PATH, params).await;
        
    // }
}
