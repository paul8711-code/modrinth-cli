use super::{API, USER_AGENT};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    hits: Vec<SearchResult>,
    offset: u32,
    limit: u32,
    total_hits: u32,
}

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    #[serde(default)]
    slug: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    categories: Vec<String>,
    #[serde(default)]
    client_side: SideSupport,
    #[serde(default)]
    server_side: SideSupport,
    project_type: String,
    downloads: u32,
    icon_url: Option<String>,
    #[serde(default)]
    color: Option<u32>,
    #[serde(default)]
    thread_id: String,
    #[serde(default)]
    monetization_status: Option<MonetizationStatus>,
    project_id: String,
    all_project_types: Vec<String>,
    author: String,
    #[serde(default)]
    display_categories: Vec<String>,
    versions: Vec<String>,
    follows: u32,
    // ISO-8601
    date_created: String,
    date_modified: String,
    #[serde(default)]
    latest_version: String,
    license: String,
    #[serde(default)]
    gallery: Vec<String>,
    featured_gallery: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum SideSupport {
    Required,
    Optional,
    Unsupported,
    #[default]
    Unknown,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MonetizationStatus {
    Monetized,
    Demonetized,
    #[serde(alias = "force-demonetized")]
    ForceDemonetized,
}

pub fn search(query: &str) -> Result<SearchResponse, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()?;

    let index = "relevance";
    let offset = "0";
    let limit = "1";
    // SUPPORTED BY US:
    // project_type
    // all_project_types
    // categories
    // versions
    // client_side
    // server_side
    // open_source
    // (needs to be put in enum and validated)
    let facets = None;

    let mut q = vec![("query", query)];
    // will be an enum, could be none
    if let Some(facets) = facets {
        q.push(("facets", facets));
    }
    // will be an enum with default value, will never be an option
    q.push(("index", index));
    // will have defaults set
    q.push(("offset", offset));
    q.push(("limit", limit));

    // check for 200/400 error
    client
        .get(format!("{}/v2/search", API))
        .query(&q)
        .send()?
        .json()
}
