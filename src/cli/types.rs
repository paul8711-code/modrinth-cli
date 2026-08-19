use clap::ValueEnum;
use ferinth::structures::project::ProjectType as FerinthProjectType;
use ferinth::structures::search::{Facet, Sort as FerinthSort};

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum Sort {
    Relevance,
    Downloads,
    Follows,
    Newest,
    Updated,
}

impl From<Sort> for FerinthSort {
    fn from(sort: Sort) -> Self {
        match sort {
            Sort::Relevance => FerinthSort::Relevance,
            Sort::Downloads => FerinthSort::Downloads,
            Sort::Follows => FerinthSort::Follows,
            Sort::Newest => FerinthSort::Newest,
            Sort::Updated => FerinthSort::Updated,
        }
    }
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
#[clap(rename_all = "lowercase")]
pub enum Loader {
    Fabric,
    Forge,
    NeoForge,
    Quilt,
}

impl std::fmt::Display for Loader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let loader = match self {
            Self::Fabric => "fabric",
            Self::Forge => "forge",
            Self::NeoForge => "neoforge",
            Self::Quilt => "quilt",
        };

        write!(f, "{}", loader)
    }
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
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
    WorldGeneration,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let category = match self {
            Self::Adventure => "adventure",
            Self::Cursed => "cursed",
            Self::Decoration => "decoration",
            Self::Economy => "economy",
            Self::Equipment => "equipment",
            Self::Food => "food",
            Self::GameMechanics => "game-mechanics",
            Self::Library => "library",
            Self::Magic => "magic",
            Self::Management => "management",
            Self::Minigame => "minigame",
            Self::Mobs => "mobs",
            Self::Optimization => "optimization",
            Self::Social => "social",
            Self::Storage => "storage",
            Self::Technology => "technology",
            Self::Transportation => "transportation",
            Self::Utility => "utility",
            Self::WorldGeneration => "worldgen",
        };

        write!(f, "{}", category)
    }
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum ProjectType {
    Mod,
    ModPack,
    ResourcePack,
    DataPack,
    Plugin,
    Shader,
}

impl From<ProjectType> for FerinthProjectType {
    fn from(project_type: ProjectType) -> Self {
        match project_type {
            ProjectType::Mod => FerinthProjectType::Mod,
            ProjectType::ModPack => FerinthProjectType::Modpack,
            ProjectType::ResourcePack => FerinthProjectType::Resourcepack,
            ProjectType::DataPack => FerinthProjectType::Datapack,
            ProjectType::Plugin => FerinthProjectType::Plugin,
            ProjectType::Shader => FerinthProjectType::Shader,
        }
    }
}

pub fn into_facets(
    loaders: Vec<Loader>,
    categories: Vec<Category>,
    project_type: Option<ProjectType>,
    versions: Vec<String>,
    client_side: bool,
    server_side: bool,
) -> Vec<Vec<Facet>> {
    let mut facets = Vec::new();

    if loaders.is_empty() {
        // set default loaders (fabric, forge, quilt, neoforge) for compat
        facets.push(vec![
            Facet::Categories("fabric".to_string()),
            Facet::Categories("forge".to_string()),
            Facet::Categories("neoforge".to_string()),
            Facet::Categories("quilt".to_string()),
        ]);
    } else {
        facets.push(
            loaders
                .into_iter()
                .map(|l| Facet::Categories(l.to_string()))
                .collect(),
        );
    }

    let categories: Vec<Vec<Facet>> = categories
        .into_iter()
        .map(|c| vec![Facet::Categories(c.to_string())])
        .collect();
    facets.extend(categories);

    if let Some(project_type) = project_type {
        facets.push(vec![Facet::ProjectType(project_type.into())]);
    }

    let versions: Vec<Facet> = versions.into_iter().map(Facet::Versions).collect();
    facets.push(versions);

    let side_type = match (client_side, server_side) {
        (true, true) => Some("client_and_server"),
        (true, false) => Some("client_only"),
        (false, true) => Some("server_only"),
        (false, false) => None,
    };

    if let Some(side_type) = side_type {
        facets.push(vec![Facet::Custom {
            _type: "environment".to_string(),
            operation: ":".to_string(),
            value: side_type.to_string(),
        }]);
    }

    facets
}
