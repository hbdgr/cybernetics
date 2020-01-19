use crypto::hash::GenericHash;
use crypto::strings;

use database::object_queries;
use database::schema::relations;
use primitives::relation::{Relation, RelationBase};

use diesel::pg::PgConnection;
use diesel::prelude::QueryResult;

use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct DatabaseRelationBase {
    pub definition: GenericHash,
    pub first_object: GenericHash,
    pub second_object: GenericHash,
}

impl DatabaseRelationBase {
    pub fn to_relation_base(&self, connection: &PgConnection) -> QueryResult<RelationBase> {
        let def_object = object_queries::get(self.definition.clone(), &connection)?;
        let first_object = object_queries::get(self.first_object.clone(), &connection)?;
        let second_object = object_queries::get(self.second_object.clone(), &connection)?;

        Ok(RelationBase {
            definition: def_object,
            first_object: first_object,
            second_object: second_object,
        })
    }

    pub fn to_database_relation(&self, connection: &PgConnection) -> QueryResult<DatabaseRelation> {
        let relation_base = self.to_relation_base(connection)?;
        Ok(DatabaseRelation::from_relation_base(relation_base))
    }
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "relations"]
pub struct DatabaseRelation {
    pub hash: Vec<u8>,
    pub definition: Vec<u8>,
    pub first_object: Vec<u8>,
    pub second_object: Vec<u8>,
}

impl DatabaseRelation {
    pub fn hash(&self) -> GenericHash {
        GenericHash::from_bytes(&self.hash)
    }

    pub fn from_relation(relation: Relation) -> DatabaseRelation {
        DatabaseRelation {
            hash: relation.raw_hash(),
            definition: relation.relation_base.definition.raw_hash(),
            first_object: relation.relation_base.first_object.raw_hash(),
            second_object: relation.relation_base.second_object.raw_hash(),
        }
    }

    pub fn from_relation_base(relation_base: RelationBase) -> DatabaseRelation {
        let hash = relation_base.raw_hash().unwrap();

        DatabaseRelation {
            hash: hash,
            definition: relation_base.definition.raw_hash(),
            first_object: relation_base.first_object.raw_hash(),
            second_object: relation_base.second_object.raw_hash(),
        }
    }

    pub fn to_relation_base(&self, connection: &PgConnection) -> QueryResult<RelationBase> {
        self.to_database_relation_base()
            .to_relation_base(connection)
    }

    pub fn to_database_relation_base(&self) -> DatabaseRelationBase {
        DatabaseRelationBase {
            definition: GenericHash::from_bytes(&self.definition),
            first_object: GenericHash::from_bytes(&self.first_object),
            second_object: GenericHash::from_bytes(&self.second_object),
        }
    }

    pub fn to_relation(&self, connection: &PgConnection) -> QueryResult<Relation> {
        let relation_base = self.to_relation_base(connection)?;
        Ok(Relation {
            hash: self.hash(),
            relation_base: relation_base,
        })
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
