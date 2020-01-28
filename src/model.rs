use regex::Regex;
use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ContentType(pub String);

#[derive(Debug, Clone, Deserialize)]
pub struct CacheControl(pub String);

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Meta {
    pub content_type: Option<ContentType>,
    pub cache_control: Option<CacheControl>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Action {
    Upload,
    Exclude,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Instruction {
    #[serde(with = "serde_regex")]
    pub pattern: Regex,
    #[serde(default = "Default::default")]
    pub meta: Meta,
    pub action: Action,
}

#[derive(Debug, Clone)]
pub struct ObjectKey(pub String);

#[derive(Debug, Clone)]
pub struct BucketName(pub String);
