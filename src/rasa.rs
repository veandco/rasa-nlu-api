#[derive(Serialize, Deserialize)]
pub struct RasaNLUData {
    pub common_examples: Vec<CommonExample>,
    pub regex_features: Vec<RegexFeature>,
    pub entity_synonyms: Vec<EntitySynonym>,
}

impl RasaNLUData {
    pub fn new() -> Self {
        Self {
            common_examples: Vec::new(),
            regex_features: Vec::new(),
            entity_synonyms: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CommonExample {
    pub text: String,
    pub intent: String,
    pub entities: Vec<Entity>,
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
    start: i32,
    end: i32,
    value: String,
    entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegexFeature {
    pub name: String,
    pub pattern: String,
}

#[derive(Serialize, Deserialize)]
pub struct EntitySynonym {
    pub value: String,
    pub synonyms: Vec<String>,
}