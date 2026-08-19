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

use clap::{Parser, Subcommand};

pub mod types;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmds,
}

#[derive(Subcommand)]
pub enum Cmds {
    Project {
        #[command(subcommand)]
        action: ProjectAction,
    },
}

#[derive(Subcommand)]
pub enum ProjectAction {
    Search {
        query: String,
        #[arg(short, long, value_enum, default_value = "relevance")]
        sort: types::Sort,
        #[arg(short, long, value_enum)]
        loader: Vec<types::Loader>,
        #[arg(short, long, value_enum)]
        category: Vec<types::Category>,
        #[arg(short, long, value_enum)]
        project_type: Option<types::ProjectType>,
        #[arg(short, long)]
        version: Vec<String>,
        #[arg(long)]
        client_side: bool,
        #[arg(long)]
        server_side: bool,
    },
}
