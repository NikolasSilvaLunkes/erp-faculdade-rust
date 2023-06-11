// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        nome -> Varchar,
        sobrenome -> Varchar,
        cpf -> Nullable<Bpchar>,
        rg -> Nullable<Varchar>,
        data_nascimento -> Nullable<Timestamp>,
        sexo -> Nullable<Bpchar>,
        estado_civil -> Nullable<Varchar>,
        telefone -> Nullable<Varchar>,
        email -> Varchar,
        password -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}
