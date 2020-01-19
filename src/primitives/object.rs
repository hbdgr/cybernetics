use crypto::hash::GenericHash;
use database::object::DatabaseObject;
use primitives::header::{Header, ObjectType};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub header: Header,
    pub body: String,
}

impl Content {
    pub fn hash(&self) -> Result<GenericHash, ()> {
        let content_json = json!(&self);
        let bytes = serde_json::to_vec(&content_json).unwrap();
        Ok(GenericHash::new(&bytes)?)
    }

    pub fn raw_hash(&self) -> Result<Vec<u8>, ()> {
        let gen = self.hash()?;
        Ok(gen.to_vec())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Object {
    pub hash: GenericHash,
    pub content: Content,
}

impl Object {
    pub fn from_content(content: Content) -> Result<Object, ()> {
        let hash = content.hash()?;

        Ok(Object {
            hash: hash,
            content: content,
        })
    }

    pub fn from_database_object(database_object: DatabaseObject) -> Result<Object, String> {
        let content: Content = match serde_json::from_value(database_object.content) {
            Ok(ctx) => ctx,
            Err(err) => return Err(err.to_string()),
        };
        Ok(Object::from_content(content).unwrap())
    }

    // for relation definition object returns its directed property
    pub fn directed(&self) -> Option<bool> {
        let object_type = &self.content.header.object_type;
        match object_type {
            ObjectType::RelationDefinition { directed } => Some(*directed),
            _ => None,
        }
    }

    pub fn raw_hash(&self) -> Vec<u8> {
        let gen = self.hash.clone();
        gen.to_vec()
    }
}
