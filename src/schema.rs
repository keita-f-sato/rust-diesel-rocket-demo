table! {
    account (account_id) {
        account_id -> Integer,
        name -> Varchar,
        pw -> Varchar,
    }
}

table! {
    image_master (image_id) {
        image_id -> Integer,
        account_id -> Nullable<Integer>,
        image_base64 -> Nullable<Longtext>,
        delete_flag -> Nullable<Tinyint>,
    }
}

joinable!(image_master -> account (account_id));

allow_tables_to_appear_in_same_query!(
    account,
    image_master,
);
