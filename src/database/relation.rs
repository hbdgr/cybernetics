use crypto::strings;
use database::schema::relations;
use primitives::relation::{Relation, RelationBase};
use std::fmt;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "relations"]
pub struct DatabaseRelation {
    pub hash: Vec<u8>,
    pub definition: Vec<u8>,
    pub first_object: Vec<u8>,
    pub second_object: Vec<u8>,
}

impl DatabaseRelation {
    pub fn from_relation(relation: Relation) -> DatabaseRelation {
        DatabaseRelation {
            hash: relation.hash.to_vec(),
            definition: relation.relation_base.definition.to_vec(),
            first_object: relation.relation_base.first_object.to_vec(),
            second_object: relation.relation_base.second_object.to_vec(),
        }
    }

    pub fn from_relation_base(relation_base: RelationBase) -> DatabaseRelation {
        let hash = relation_base.raw_hash().unwrap();

        DatabaseRelation {
            hash: hash,
            definition: relation_base.definition.to_vec(),
            first_object: relation_base.first_object.to_vec(),
            second_object: relation_base.second_object.to_vec(),
        }
    }
}

impl fmt::Display for DatabaseRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "hash: {}, object_definition: {}, first_object: {}, second_object: {}",
            strings::to_hex_string(&self.hash),
            strings::to_hex_string(&self.definition),
            strings::to_hex_string(&self.first_object),
            strings::to_hex_string(&self.second_object),
        )
    }
}
