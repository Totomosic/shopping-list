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
        username -> Bpchar,
        password_hash -> Bpchar,
        is_admin -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    spatial_ref_sys,
    users,
);
