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

//// posts and teams
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivityRoot {
    pub activities: Vec<Activity>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Activity {
    pub action: String,
    pub content: String,
    pub date: String,
    pub id: i64,
    pub name: String,
    pub project_id: i64,
    pub project_name: String,
    pub project_phase: String,
    pub project_score: i64,
    pub ref_url: String,
    pub time: i64,
    pub timesince: String,
    pub user_id: i64,
    pub user_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectsRoot {
    pub event: Event,
    pub projects: Vec<Project>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub community_url: String,
    pub ends_at: String,
    pub gallery_url: String,
    pub has_finished: bool,
    pub has_started: bool,
    pub hashtags: String,
    pub hostname: String,
    pub id: i64,
    pub location: String,
    pub location_lat: f64,
    pub location_lon: f64,
    pub logo_url: String,
    pub name: String,
    pub starts_at: String,
    pub summary: String,
    pub webpage_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub autotext_url: String,
    pub category_id: String,
    pub category_name: String,
    pub contact_url: String,
    pub created_at: String,
    pub download_url: String,
    pub event_name: String,
    pub event_url: String,
    pub excerpt: String,
    pub hashtag: String,
    pub id: i64,
    pub ident: String,
    pub image_url: String,
    pub is_challenge: bool,
    pub is_webembed: bool,
    pub logo_color: String,
    pub logo_icon: String,
    pub maintainer: String,
    pub name: String,
    pub phase: String,
    pub progress: i64,
    pub score: i64,
    pub source_url: String,
    #[serde(rename = "stats-commits")]
    pub stats_commits: i64,
    #[serde(rename = "stats-during")]
    pub stats_during: i64,
    #[serde(rename = "stats-people")]
    pub stats_people: i64,
    #[serde(rename = "stats-sizepitch")]
    pub stats_sizepitch: i64,
    #[serde(rename = "stats-sizetotal")]
    pub stats_sizetotal: i64,
    #[serde(rename = "stats-total")]
    pub stats_total: i64,
    #[serde(rename = "stats-updates")]
    pub stats_updates: i64,
    pub summary: String,
    pub team: String,
    pub team_count: i64,
    pub updated_at: String,
    pub url: String,
    pub webpage_url: String,
}