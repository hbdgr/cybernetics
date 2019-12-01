use database::schema::relations;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "relations"]
pub struct Relation {
    pub id: i64,
    pub object_definition_id: i64,
    pub first_object_id: i64,
    pub second_object_id: i64,
}

impl Relation {
    pub fn from_insertable_object(id: i64, insertable: InsertableRelation) -> Relation {
        Relation {
            id: id,
            object_definition_id: insertable.object_definition_id,
            first_object_id: insertable.first_object_id,
            second_object_id: insertable.second_object_id,
        }
    }
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "relations"]
pub struct InsertableRelation {
    pub object_definition_id: i64,
    pub first_object_id: i64,
    pub second_object_id: i64,
}

impl InsertableRelation {
    pub fn from_relation(relation: Relation) -> InsertableRelation {
        InsertableRelation {
            object_definition_id: relation.object_definition_id,
            first_object_id: relation.first_object_id,
            second_object_id: relation.second_object_id,
        }
    }
}
