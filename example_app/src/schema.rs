// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        cost -> Float8,
        active -> Bool,
    }
}

diesel::table! {
    products_variants (id) {
        id -> Int4,
        variant_id -> Int4,
        product_id -> Int4,
        value -> Nullable<Varchar>,
    }
}

diesel::table! {
    variants (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(products_variants -> products (product_id));
diesel::joinable!(products_variants -> variants (variant_id));

diesel::allow_tables_to_appear_in_same_query!(
    products,
    products_variants,
    variants,
);
