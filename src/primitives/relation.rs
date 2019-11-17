#[derive(Serialize, Deserialize, Debug)]
pub struct Relation {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub relation_object_id: i64,
    pub first_object_id: i64,
    pub second_object_id: i64,
}
