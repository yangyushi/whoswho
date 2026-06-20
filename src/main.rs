mod cli;
mod commands;
mod db;
mod errors;
mod models;

use crate::cli::{Cli, Commands};
use crate::db::Database;
use crate::errors::{Result, WswError};
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
        Some(Commands::Get { name, id, json }) => {
            commands::get::run(&db, name, id, json || cli.json)?;
        }
        Some(Commands::Add { name, fields }) => {
            reject_root_json(cli.json)?;
            commands::add::run(&db, name, fields)?;
        }
        Some(Commands::Set { name, fields, id }) => {
            reject_root_json(cli.json)?;
            commands::set::run(&db, name, fields, id)?;
        }
        Some(Commands::Note { name, content, id }) => {
            reject_root_json(cli.json)?;
            commands::note::run(&db, name, content, id)?;
        }
        Some(Commands::Log { name, id, limit }) => {
            reject_root_json(cli.json)?;
            commands::log::run(&db, name, id, limit)?;
        }
        Some(Commands::List { limit, json }) => {
            commands::list::run(&db, limit, json || cli.json)?;
        }
        Some(Commands::Search { query, field, json }) => {
            commands::search::run(&db, query, field, json || cli.json)?;
        }
        Some(Commands::Rm {
            name,
            field,
            id,
            yes,
        }) => {
            reject_root_json(cli.json)?;
            commands::rm::run(&db, name, field, id, yes)?;
        }
        None => {
            if let Some(name) = cli.name {
                let use_id = name.parse::<i64>().is_ok();
                commands::get::run(&db, name, use_id, cli.json)?;
            } else {
                let mut cmd = Cli::command();
                cmd.print_help()?;
                println!();
            }
        }
    }

    Ok(())
}

fn reject_root_json(json: bool) -> Result<()> {
    if json {
        Err(WswError::Other(
            "--json is only supported for quick lookup, get, list, and search".to_string(),
        ))
    } else {
        Ok(())
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
