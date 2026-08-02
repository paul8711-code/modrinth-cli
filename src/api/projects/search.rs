/*
    modrinth-cli  An easy-to-use, powerful Rust CLI to manage your local Minecraft instances and Modrinth mods
    Copyright (C) 2026  Paul8711

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as
    published by the Free Software Foundation, either version 3 of the
    License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use super::{API, USER_AGENT};
use clap::ValueEnum;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

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
    categories: Vec<CategoryOrLoader>,
    #[serde(default)]
    client_side: SideSupport,
    #[serde(default)]
    server_side: SideSupport,
    project_type: ProjectType,
    downloads: u32,
    icon_url: Option<String>,
    #[serde(default)]
    color: Option<u32>,
    #[serde(default)]
    thread_id: String,
    #[serde(default)]
    monetization_status: Option<MonetizationStatus>,
    project_id: String,
    all_project_types: Vec<ProjectType>,
    author: String,
    #[serde(default)]
    display_categories: Vec<CategoryOrLoader>,
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

#[derive(Default, Serialize, Clone, Debug, PartialEq, ValueEnum)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "kebab-case")]
pub enum Sort {
    #[default]
    Relevance,
    Downloads,
    Follows,
    Newest,
    Updated,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, ValueEnum)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "lowercase")]
pub enum Loader {
    Fabric,
    Forge,
    NeoForge,
    Quilt,
}

impl Loader {
    pub fn as_str(&self) -> &'static str {
        match self {
            Loader::Fabric => "Fabric",
            Loader::Forge => "Forge",
            Loader::NeoForge => "NeoForge",
            Loader::Quilt => "Quilt",
        }
    }

    pub fn to_slug(&self) -> &'static str {
        match self {
            Loader::Fabric => "fabric",
            Loader::Forge => "forge",
            Loader::NeoForge => "neoforge",
            Loader::Quilt => "quilt",
        }
    }
}

impl std::fmt::Display for Loader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, ValueEnum)]
#[serde(rename_all = "kebab-case")]
#[clap(rename_all = "kebab-case")]
pub enum Category {
    Adventure,
    Cursed,
    Decoration,
    Economy,
    Equipment,
    Food,
    GameMechanics,
    Library,
    Magic,
    Management,
    Minigame,
    Mobs,
    Optimization,
    Social,
    Storage,
    Technology,
    Transportation,
    Utility,
    #[serde(rename = "worldgen")]
    WorldGeneration,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Adventure => "Adventure",
            Category::Cursed => "Cursed",
            Category::Decoration => "Decoration",
            Category::Economy => "Economy",
            Category::Equipment => "Equipment",
            Category::Food => "Food",
            Category::GameMechanics => "Game Mechanics",
            Category::Library => "Library",
            Category::Magic => "Magic",
            Category::Management => "Management",
            Category::Minigame => "Minigame",
            Category::Mobs => "Mobs",
            Category::Optimization => "Optimization",
            Category::Social => "Social",
            Category::Storage => "Storage",
            Category::Technology => "Technology",
            Category::Transportation => "Transportation",
            Category::Utility => "Utility",
            Category::WorldGeneration => "World Generation",
        }
    }

    pub fn to_slug(&self) -> &'static str {
        match self {
            Category::GameMechanics => "game-mechanics",
            Category::WorldGeneration => "worldgen",
            Category::Adventure => "adventure",
            Category::Cursed => "cursed",
            Category::Decoration => "decoration",
            Category::Economy => "economy",
            Category::Equipment => "equipment",
            Category::Food => "food",
            Category::Library => "library",
            Category::Magic => "magic",
            Category::Management => "management",
            Category::Minigame => "minigame",
            Category::Mobs => "mobs",
            Category::Optimization => "optimization",
            Category::Social => "social",
            Category::Storage => "storage",
            Category::Technology => "technology",
            Category::Transportation => "transportation",
            Category::Utility => "utility",
        }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum CategoryOrLoader {
    Category(Category),
    Loader(Loader),
    ProjectType(ProjectType),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, ValueEnum)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "kebab-case")]
pub enum ProjectType {
    Mod,
    ModPack,
    ResourcePack,
    DataPack,
    Plugin,
    Shader,
}

impl ProjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectType::Mod => "Mod",
            ProjectType::ModPack => "Modpack",
            ProjectType::ResourcePack => "Resource Pack",
            ProjectType::DataPack => "Data Pack",
            ProjectType::Plugin => "Plugin",
            ProjectType::Shader => "Shader",
        }
    }

    pub fn to_slug(&self) -> &'static str {
        match self {
            ProjectType::Mod => "mod",
            ProjectType::ModPack => "modpack",
            ProjectType::ResourcePack => "resourcepack",
            ProjectType::DataPack => "datapack",
            ProjectType::Plugin => "plugin",
            ProjectType::Shader => "shader",
        }
    }
}

impl std::fmt::Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// SUPPORTED BY US:
// all_project_types
// categories
// versions
// client_side
// server_side
#[derive(Serialize, PartialEq, Clone)]
pub enum Filter {
    Loader(Loader),
    Category(Category),
    Version(String),
    // if omitted: no filter, if false: optional filter, if true: required filter
    ServerSide(bool),
    ClientSide(bool),
    ProjectType(ProjectType),
}

impl Filter {
    pub fn to_facet_string(&self) -> String {
        match self {
            Filter::Loader(loader) => {
                format!("categories:{}", loader.to_slug())
            }
            Filter::Category(category) => {
                format!("categories:{}", category.to_slug())
            }
            // dont need to validate, api returns nothing when non existent
            Filter::Version(ver) => format!("versions:{ver}"),
            Filter::ServerSide(req) => {
                let status = if *req { "required" } else { "optional" };
                format!("server_side:{status}")
            }
            Filter::ClientSide(req) => {
                let status = if *req { "required" } else { "optional" };
                format!("client_side:{status}")
            }
            Filter::ProjectType(p_type) => {
                format!("all_project_types:{}", p_type.to_slug())
            }
        }
    }
}

fn serialize_filters(filters: &[Filter]) -> Option<String> {
    let has_loader = filters.iter().any(|f| matches!(f, Filter::Loader(_)));

    let mut facet_groups: Vec<Vec<String>> = filters
        .iter()
        .filter(|f| !matches!(f, Filter::Loader(_)))
        .map(|f| vec![f.to_facet_string()])
        .collect();

    if has_loader {
        let loaders = filters
            .iter()
            .filter_map(|f| match f {
                Filter::Loader(_) => Some(f.to_facet_string()),
                _ => None,
            })
            .collect();

        facet_groups.push(loaders);
    } else {
        facet_groups.push(vec![
            Filter::Loader(Loader::Fabric).to_facet_string(),
            Filter::Loader(Loader::Forge).to_facet_string(),
            Filter::Loader(Loader::NeoForge).to_facet_string(),
            Filter::Loader(Loader::Quilt).to_facet_string(),
        ]);
    }

    serde_json::to_string(&facet_groups).ok()
}

pub fn search(
    query: &str,
    sort: Sort,
    offset: u32,
    limit: u32,
    filter: &[Filter],
) -> Result<SearchResponse, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()?;

    let facets = serialize_filters(filter);

    let offset = offset.to_string();
    let limit = limit.to_string();

    client
        .get(format!("{}/v2/search", API))
        .query(&[("query", query), ("offset", &offset), ("limit", &limit)])
        .query(&[("index", sort)])
        .query(&[("facets", facets)])
        .send()?
        .json()
}
