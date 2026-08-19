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

use clap::Parser;
use cli::{Cli, Cmds, ProjectAction};

mod cli;

#[tokio::main]
async fn main() {
    let c = Cli::parse();

    let modrinth = ferinth::Ferinth::<()>::new(
        env!("CARGO_PKG_NAME"),
        Some(env!("CARGO_PKG_VERSION")),
        Some(env!("CARGO_PKG_AUTHORS")),
    );

    match c.cmd {
        Cmds::Project { action } => match action {
            ProjectAction::Search {
                query,
                sort,
                loader,
                category,
                project_type,
                version,
                client_side,
                server_side,
            } => {
                let response = modrinth
                    .search_paged(
                        &query,
                        sort.into(),
                        1,
                        0,
                        cli::types::into_facets(
                            loader,
                            category,
                            project_type,
                            version,
                            client_side,
                            server_side,
                        ),
                    )
                    .await
                    .unwrap();
                println!("{:?}", response);
            }
        },
    }
}
