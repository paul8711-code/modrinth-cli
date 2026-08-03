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
use cli::{Cli, Cmds, ProjectsAction};

use api::projects::search::Filter;

mod api;
mod cli;

fn main() {
    let c = Cli::parse();

    match c.cmd {
        Cmds::Projects { action } => match action {
            ProjectsAction::Search {
                query,
                sort,
                loader,
                category,
                project_type,
                version,
                client_side,
                server_side,
            } => {
                let loader: Vec<Filter> = loader
                    .into_iter()
                    .map(|l| Filter::Loader(l.clone()))
                    .collect();

                let category: Vec<Filter> = category
                    .into_iter()
                    .map(|c| Filter::Category(c.clone()))
                    .collect();

                let project_type: Vec<Filter> = project_type
                    .into_iter()
                    .map(|p| Filter::ProjectType(p.clone()))
                    .collect();

                let version: Vec<Filter> = version
                    .into_iter()
                    .map(|v| Filter::Version(v.clone()))
                    .collect();

                let client_side: Vec<Filter> = if client_side {
                    vec![Filter::ClientSide]
                } else {
                    Vec::new()
                };

                let server_side: Vec<Filter> = if server_side {
                    vec![Filter::ServerSide]
                } else {
                    Vec::new()
                };

                let filter: Vec<Filter> = [
                    loader,
                    category,
                    project_type,
                    version,
                    client_side,
                    server_side,
                ]
                .concat();

                let response = match api::projects::search(&query, sort, 0, 1, &filter) {
                    Ok(r) => r,
                    Err(e) => {
                        println!("{:?}", e);
                        std::process::exit(1);
                    }
                };
                println!("{:?}", response);
            }
        },
    }
}
