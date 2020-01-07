use crypto::hash;
use crypto::hash::GenericHash;
use database::object_queries::get_disposable;
use database::relation::DatabaseRelation;

#[derive(Clone, Serialize, Deserialize)]
pub struct RelationBase {
    pub definition: GenericHash,
    pub first_object: GenericHash,
    pub second_object: GenericHash,
}

impl RelationBase {
    pub fn hash(&self) -> Result<GenericHash, ()> {
        Ok(GenericHash::from_bytes(&self.raw_hash()?))
    }

    pub fn raw_hash(&self) -> Result<Vec<u8>, ()> {
        let mut hasher = hash::generic_state()?;

        let obj1: &str = &self.first_object.to_string();
        let obj2: &str = &self.second_object.to_string();

        let mut obj_vec: Vec<&str> = vec![obj1, obj2];

        // check if obiect is directed
        get_disposable(&self.definition)
            .map(|object| {
                object.directed().map(|directed| {
                    // if not directed relation
                    // make final hash independent of the order of first and second objects
                    if !directed {
                        obj_vec.sort();
                    }
                });
            })
            .unwrap();

        hasher.update(&self.definition.to_vec())?;
        hasher.update(obj_vec[0].as_bytes())?;
        hasher.update(obj_vec[1].as_bytes())?;

        Ok(hash::generic_finalize(hasher)?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Relation {
    pub hash: GenericHash,
    pub relation_base: RelationBase,
}

impl Relation {
    pub fn from_relation_base(relation_base: RelationBase) -> Result<Relation, ()> {
        let hash = relation_base.hash()?;

        Ok(Relation {
            hash: hash,
            relation_base: relation_base,
        })
    }

    pub fn from_database_relation(database_relation: DatabaseRelation) -> Result<Relation, ()> {
        let relation_base: RelationBase = RelationBase {
            definition: GenericHash::from_bytes(&database_relation.definition),
            first_object: GenericHash::from_bytes(&database_relation.first_object),
            second_object: GenericHash::from_bytes(&database_relation.second_object),
        };

        Ok(Relation {
            hash: relation_base.hash()?,
            relation_base: relation_base,
        })
    }
}
