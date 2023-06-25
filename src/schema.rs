// @generated automatically by Diesel CLI.

diesel::table! {
    clientes (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 100]
        nome -> Varchar,
        #[max_length = 100]
        sobrenome -> Varchar,
        #[max_length = 11]
        cpf -> Nullable<Bpchar>,
        #[max_length = 20]
        rg -> Nullable<Varchar>,
        data_nascimento -> Nullable<Timestamp>,
        #[max_length = 1]
        sexo -> Nullable<Bpchar>,
        #[max_length = 20]
        estado_civil -> Nullable<Varchar>,
        #[max_length = 22]
        telefone -> Nullable<Varchar>,
        #[max_length = 36]
        created_by -> Varchar,
        created_at -> Timestamp,
        #[max_length = 36]
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    orcamento_produtos (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        id_produto -> Varchar,
        #[max_length = 36]
        id_orcamento -> Varchar,
        quantidade -> Int4,
        #[max_length = 36]
        created_by -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    orcamentos (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        created_by -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    produtos (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 100]
        descricao -> Varchar,
        #[max_length = 50]
        codigo_de_barras -> Nullable<Varchar>,
        #[max_length = 12]
        icms -> Nullable<Varchar>,
        #[max_length = 2]
        unidade_medida -> Nullable<Bpchar>,
        preco_venda -> Nullable<Numeric>,
        preco_minimo -> Nullable<Numeric>,
        custo_medio -> Nullable<Numeric>,
        #[max_length = 4]
        pis -> Nullable<Varchar>,
        #[max_length = 4]
        cofins -> Nullable<Varchar>,
        #[max_length = 8]
        ncm -> Nullable<Varchar>,
        #[max_length = 12]
        cest -> Nullable<Varchar>,
        #[max_length = 12]
        cod_ipi -> Nullable<Varchar>,
        porcentagem_ipi -> Nullable<Numeric>,
        #[max_length = 36]
        created_by -> Varchar,
        created_at -> Timestamp,
        #[max_length = 36]
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 100]
        nome -> Varchar,
        #[max_length = 100]
        sobrenome -> Varchar,
        #[max_length = 11]
        cpf -> Nullable<Bpchar>,
        #[max_length = 20]
        rg -> Nullable<Varchar>,
        data_nascimento -> Nullable<Timestamp>,
        #[max_length = 1]
        sexo -> Nullable<Bpchar>,
        #[max_length = 20]
        estado_civil -> Nullable<Varchar>,
        #[max_length = 22]
        telefone -> Nullable<Varchar>,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 122]
        password -> Varchar,
        #[max_length = 36]
        created_by -> Varchar,
        created_at -> Timestamp,
        #[max_length = 36]
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(orcamento_produtos -> orcamentos (id_orcamento));
diesel::joinable!(orcamento_produtos -> produtos (id_produto));

diesel::allow_tables_to_appear_in_same_query!(
    clientes,
    orcamento_produtos,
    orcamentos,
    produtos,
    users,
);
