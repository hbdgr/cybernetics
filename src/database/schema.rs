table! {
    objects (hash) {
        hash -> Bytea,
        content -> Json,
    }
}

table! {
    relations (hash) {
        hash -> Bytea,
        definition -> Bytea,
        first_object -> Bytea,
        second_object -> Bytea,
    }
}

allow_tables_to_appear_in_same_query!(
    objects,
    relations,
);
