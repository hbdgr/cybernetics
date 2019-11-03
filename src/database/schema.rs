table! {
    objects (id) {
        id -> Int8,
        content -> Json,
    }
}

table! {
    relations (id) {
        id -> Int8,
        relation_object_id -> Int8,
        first_object_id -> Int8,
        second_object_id -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    objects,
    relations,
);
