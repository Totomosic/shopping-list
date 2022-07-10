table! {
    shopping_item (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        image_url -> Nullable<Varchar>,
        default_unit_type -> Varchar,
    }
}

table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Int4,
        display_name -> Varchar,
        username -> Varchar,
        password_hash -> Varchar,
        is_admin -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    shopping_item,
    spatial_ref_sys,
    users,
);
