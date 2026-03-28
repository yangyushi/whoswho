mod cli;
mod commands;
mod db;
mod errors;
mod models;

use crate::cli::{Cli, Commands};
use crate::db::Database;
use crate::errors::Result;
use clap::{CommandFactory, Parser};
use std::path::PathBuf;

fn get_db_path(db_arg: Option<PathBuf>) -> PathBuf {
    db_arg.unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Could not determine home directory")
            .join(".wsw.db")
    })
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let db_path = get_db_path(cli.db);
    let db = Database::open(&db_path)?;

    match cli.command {
        Some(Commands::Get { name, id }) => {
            commands::get::run(&db, name, id, cli.json)?;
        }
        Some(Commands::Add { name, fields }) => {
            commands::add::run(&db, name, fields)?;
        }
        Some(Commands::Set { name, fields, id }) => {
            commands::set::run(&db, name, fields, id)?;
        }
        Some(Commands::Note { name, content, id }) => {
            commands::note::run(&db, name, content, id)?;
        }
        Some(Commands::Log { name, id, limit }) => {
            commands::log::run(&db, name, id, limit)?;
        }
        Some(Commands::List { recent, limit }) => {
            commands::list::run(&db, recent, limit, cli.json)?;
        }
        Some(Commands::Search { query, field }) => {
            commands::search::run(&db, query, field, cli.json)?;
        }
        Some(Commands::Rm { name, field, id }) => {
            commands::rm::run(&db, name, field, id, cli.yes)?;
        }
        None => {
            if let Some(name) = cli.name {
                commands::get::run(&db, name, false, cli.json)?;
            } else {
                let mut cmd = Cli::command();
                cmd.print_help()?;
                println!();
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
