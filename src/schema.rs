// @generated automatically by Diesel CLI.

diesel::table! {
    todo_item (id) {
        id -> Int4,
        #[max_length = 150]
        title -> Varchar,
        checked -> Bool,
        list_id -> Int4,
    }
}

diesel::table! {
    todo_list (id) {
        id -> Int4,
        #[max_length = 150]
        title -> Nullable<Varchar>,
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

diesel::joinable!(todo_item -> todo_list (list_id));

diesel::allow_tables_to_appear_in_same_query!(
    todo_item,
    todo_list,
    users,
);
