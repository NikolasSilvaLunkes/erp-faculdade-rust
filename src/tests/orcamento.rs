#[cfg(test)]
mod tests {
    use crate::handlers::orcamento::{CreateOrcamentoRequest};
    // use crate::handlers::orcamento_produto::{OrcamentoProduto, NewOrcamentoProduto};
    use crate::tests::helpers::tests::{assert_get, assert_post};
    use crate::models::cliente::delete as model_delete;
    use chrono::{NaiveDate};
    use actix_web::web::Path;
    use uuid::Uuid;

    const PATH: &str = "/api/v1/orcamento";

    // #[actix_rt::test]
    // async fn it_creates_a_orcamento() {
    //     let orcamento_produto: OrcamentoProduto = NewOrcamentoProduto{
    //         id: Uuid::new_v4().to_string(),
    //         id_produto: "00000000-0000-0000-0000-000000000000".into(),
    //         id_orcamento: "00000000-0000-0000-0000-000000000000".into(),
    //         quantidade: 1,
    //         created_by: "00000000-0000-0000-0000-000000000000".into(),
    //     }.into();
    //     let orcamentos_produtos = vec![orcamento_produto];
    //     let params = CreateOrcamentoRequest {
    //         produtos: orcamentos_produtos,
    //     };
    //     let response = assert_post(PATH, params).await;
        
    // }
}
