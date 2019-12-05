use crypto::hash;
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

    pub fn hash(&self) -> Result<Vec<u8>, ()> {
        let mut hasher = hash::generic_state()?;
        hasher.update(&self.object_definition_id.to_be_bytes())?;

        // make a hash independent of the order of first and second objects
        if self.first_object_id > self.second_object_id {
            hasher.update(&self.first_object_id.to_be_bytes())?;
            hasher.update(&self.second_object_id.to_be_bytes())?;
        } else {
            hasher.update(&self.second_object_id.to_be_bytes())?;
            hasher.update(&self.first_object_id.to_be_bytes())?;
        }

        Ok(hash::generic_finalize(hasher)?)
    }
}
