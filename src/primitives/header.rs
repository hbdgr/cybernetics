#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectType {
    PrimaryElement,
    // define relation, if use in Header, 'directed' property must be set
    RelationDefinition { directed: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub object_type: ObjectType,
}
