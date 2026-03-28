use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "wsw")]
#[command(about = "Store and retrieve personal information (who is who)")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Name to query (when no subcommand provided)
    #[arg(value_name = "NAME", global = true)]
    pub name: Option<String>,

    /// Path to database file
    #[arg(long, global = true, value_name = "PATH", env = "WSW_DB")]
    pub db: Option<PathBuf>,

    /// Output as JSON
    #[arg(long, global = true)]
    pub json: bool,

    /// Skip confirmations
    #[arg(short, long, global = true)]
    pub yes: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get information about a person
    Get {
        /// Person's name or ID
        name: String,

        /// Get by ID instead of name
        #[arg(long)]
        id: bool,
    },

    /// Add a new person
    Add {
        /// Person's name
        name: String,

        /// Initial fields (Key=Value pairs)
        #[arg(value_name = "FIELD=VALUE")]
        fields: Vec<String>,
    },

    /// Update fields for a person
    Set {
        /// Person's name or ID
        name: String,

        /// Fields to update (Key=Value pairs)
        #[arg(value_name = "FIELD=VALUE", required = true)]
        fields: Vec<String>,

        /// Update by ID instead of name
        #[arg(long)]
        id: bool,
    },

    /// Add a note to a person
    Note {
        /// Person's name or ID
        name: String,

        /// Note content
        content: String,

        /// Use ID instead of name
        #[arg(long)]
        id: bool,
    },

    /// Show notes/history for a person
    Log {
        /// Person's name or ID
        name: String,

        /// Use ID instead of name
        #[arg(long)]
        id: bool,

        /// Limit number of notes
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },

    /// List all people
    List {
        /// Show recently updated first
        #[arg(long)]
        recent: bool,

        /// Limit number of results
        #[arg(short, long)]
        limit: Option<usize>,
    },

    /// Search for people
    Search {
        /// Search query
        query: String,

        /// Search specific field
        #[arg(short, long, value_name = "FIELD")]
        field: Option<String>,
    },

    /// Remove a person or field
    Rm {
        /// Person's name or ID
        name: String,

        /// Remove specific field instead of person
        #[arg(long, value_name = "FIELD")]
        field: Option<String>,

        /// Use ID instead of name
        #[arg(long)]
        id: bool,
    },
}
