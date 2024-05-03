use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Datapackage {
    pub contributors: Vec<Contributor>,
    pub created: String,
    pub description: String,
    pub homepage: String,
    pub keywords: Vec<String>,
    pub licenses: Vec<License>,
    pub name: String,
    pub resources: Vec<Resource>,
    pub sources: Vec<Source>,
    pub title: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contributor {
    pub path: String,
    pub role: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct License {
    pub name: String,
    pub path: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    pub data: Vec<Daum>,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum {
    pub aftersubmit: Option<String>,
    pub boilerplate: Option<String>,
    pub certificate_path: Option<String>,
    pub community_embed: Option<String>,
    pub community_url: Option<String>,
    pub custom_css: Option<String>,
    pub description: Option<String>,
    pub ends_at: Option<String>,
    pub gallery_url: Option<String>,
    pub has_finished: Option<bool>,
    pub has_started: Option<bool>,
    pub hashtags: Option<String>,
    pub hostname: Option<String>,
    pub id: i64,
    pub instruction: Option<String>,
    pub location: Option<String>,
    pub location_lat: Option<f64>,
    pub location_lon: Option<f64>,
    pub logo_url: Option<String>,
    pub name: String,
    pub starts_at: Option<String>,
    pub summary: String,
    pub webpage_url: String,
    pub autotext: Option<String>,
    pub autotext_url: Option<String>,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub contact_url: Option<String>,
    pub created_at: Option<String>,
    pub download_url: Option<String>,
    pub event_name: Option<String>,
    pub event_url: Option<String>,
    pub excerpt: Option<String>,
    pub hashtag: Option<String>,
    pub ident: Option<Value>,
    pub image_url: Option<String>,
    pub is_challenge: Option<bool>,
    pub is_webembed: Option<bool>,
    pub logo_color: Option<String>,
    pub logo_icon: Option<String>,
    pub longtext: Option<String>,
    pub maintainer: Option<String>,
    pub phase: Option<String>,
    pub progress: Option<i64>,
    pub score: Option<i64>,
    pub source_url: Option<String>,
    pub stats: Option<Stats>,
    pub team: Option<String>,
    pub team_count: Option<i64>,
    pub updated_at: Option<String>,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub commits: i64,
    pub during: i64,
    pub people: i64,
    pub sizepitch: i64,
    pub sizetotal: i64,
    pub total: i64,
    pub updates: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub path: String,
    pub title: String,
}
