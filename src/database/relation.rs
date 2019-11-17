use database::schema::relation;
use primitives::relation::Relation;
use serde_json::json;

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "relation"]
pub struct InsertableRelation {
    pub relation_object_id: i64,
    pub first_object_id: i64,
    pub second_object_id: i64,
}

impl InsertableRelation {
    pub fn from_relation(relation: Relation) -> InsertableRelation {
        InsertableRelation {
            relation_object_id: Relation.relation_object_id,
            first_object_id: Relation.first_object_id,
            second_object_id: Relation.second_object_id,
        }
    }
}
