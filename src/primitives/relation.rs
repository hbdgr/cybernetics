use crypto::hash;
use crypto::hash::GenericHash;
use primitives::object::Object;

#[derive(Clone, Serialize, Deserialize)]
pub struct RelationBase {
    pub definition: Object,
    pub first_object: Object,
    pub second_object: Object,
}

impl RelationBase {
    pub fn hash(&self) -> Result<GenericHash, ()> {
        Ok(GenericHash::from_bytes(&self.raw_hash()?))
    }

    pub fn raw_hash(&self) -> Result<Vec<u8>, ()> {
        let mut hasher = hash::generic_state()?;

        let obj1: &str = &self.first_object.hash.to_string();
        let obj2: &str = &self.second_object.hash.to_string();

        let mut obj_vec: Vec<&str> = vec![obj1, obj2];

        // check if obiect is directed
        self.definition.directed().map(|directed| {
            // if not directed relation
            // make final hash independent of the order of first and second objects
            if !directed {
                obj_vec.sort();
            }
        });

        hasher.update(&self.definition.hash.to_vec())?;
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

    pub fn raw_hash(&self) -> Vec<u8> {
        let gen = self.hash.clone();
        gen.to_vec()
    }
}
