const USER_AGENT: &str = concat!(
    "paul8711-code/",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_AUTHORS"),
    ")"
);

const API: &str = "https://api.modrinth.com";

// api endpoints
pub mod projects;
