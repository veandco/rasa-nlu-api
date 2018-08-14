#[derive(Serialize, Deserialize)]
pub struct RasaNLU {
    pub rasa_nlu_data: RasaNLUData,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct CommonExample {
    pub text: String,
    pub intent: String,
    pub entities: Vec<Entity>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity {
    start: i32,
    end: i32,
    value: String,
    entity: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegexFeature {
    pub name: String,
    pub pattern: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EntitySynonym {
    pub value: String,
    pub synonyms: Vec<String>,
}