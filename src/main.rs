mod commands;
mod storage;
mod task;

use clap::{Parser, Subcommand};
use task::{Priority, Status};

#[derive(Parser)]
#[command(name = "kaam")]
#[command(about = "A simple CLI todo application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Task description
        description: String,
        /// Priority level (low, medium, high)
        #[arg(short, long)]
        priority: Option<Priority>,
        /// Due date (YYYY-MM-DD format)
        #[arg(short, long)]
        due: Option<String>,
    },
    /// List all tasks
    List {
        /// Filter by status (pending, done)
        #[arg(short, long)]
        status: Option<Status>,
        /// Filter by priority (low, medium, high)
        #[arg(short, long)]
        priority: Option<Priority>,
    },
    /// Mark a task as done
    Done {
        /// Task ID to mark as done
        id: u32,
    },
    /// Edit an existing task
    Edit {
        /// Task ID to edit
        id: u32,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
        /// New priority (low, medium, high)
        #[arg(short, long)]
        priority: Option<Priority>,
        /// New due date (YYYY-MM-DD format)
        #[arg(long)]
        due: Option<String>,
    },
    /// Delete a task
    Delete {
        /// Task ID to delete
        id: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Add {
            description,
            priority,
            due,
        } => commands::add_task(description, priority, due),
        Commands::List { status, priority } => commands::list_tasks(status, priority),
        Commands::Done { id } => commands::mark_done(id),
        Commands::Edit {
            id,
            description,
            priority,
            due,
        } => commands::edit_task(id, description, priority, due),
        Commands::Delete { id } => commands::delete_task(id),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
