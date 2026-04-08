use clap::{Parser, Subcommand};
use iced_layout_inspector::server::client;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "iced-debug")]
#[command(about = "Debug tool for iced applications")]
struct Cli {
    /// Socket path (auto-detects if not specified)
    #[arg(short, long, global = true)]
    socket: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Dump the current layout
    Dump,
    /// Set text in a field (by placeholder)
    Input {
        /// Field placeholder text
        field: String,
        /// Value to set
        value: String,
    },
    /// Click a button by label
    Click {
        /// Button label
        label: String,
    },
    /// Submit form (press Enter)
    Submit,
    /// Send a key press event (e.g., "Escape", "Return", "t")
    Key {
        /// Key name
        key: String,
    },
    /// Ping the app
    Ping,
    /// List running iced debug servers
    List,
    /// Take a screenshot and save to file
    Screenshot {
        /// Output file path (JPEG format)
        #[arg(default_value = "screenshot.jpg")]
        output: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Commands::List = cli.command {
        list_servers();
        return;
    }

    let Some(socket) = resolve_socket(cli.socket) else {
        return;
    };
    run_command(cli.command, &socket);
}

fn resolve_socket(explicit: Option<PathBuf>) -> Option<PathBuf> {
    if let Some(s) = explicit {
        return Some(s);
    }
    let servers = client::find_servers();
    match servers.len() {
        1 => Some(servers.into_iter().next().unwrap()),
        0 => {
            eprintln!("No iced debug servers found");
            None
        }
        _ => {
            eprintln!("Multiple servers found, specify --socket:");
            for s in &servers {
                eprintln!("  {}", s.display());
            }
            None
        }
    }
}

fn list_servers() {
    let servers = client::find_servers();
    if servers.is_empty() {
        println!("No iced debug servers running");
    } else {
        for s in servers {
            println!("{}", s.display());
        }
    }
}

fn cmd_dump(socket: &PathBuf) {
    match client::dump(socket) {
        Ok(layout) => println!("{}", layout),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_input(socket: &PathBuf, field: &str, value: &str) {
    match client::input(socket, field, value) {
        Ok(()) => println!("OK"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_click(socket: &PathBuf, label: &str) {
    match client::click(socket, label) {
        Ok(()) => println!("OK"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_submit(socket: &PathBuf) {
    match client::submit(socket) {
        Ok(()) => println!("OK"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_key(socket: &PathBuf, key: &str) {
    match client::key(socket, key) {
        Ok(()) => println!("Sent key '{}'", key),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_ping(socket: &PathBuf) {
    match client::ping(socket) {
        Ok(()) => println!("Pong"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_screenshot(socket: &PathBuf, output: &PathBuf) {
    match client::screenshot_to_file(socket, output) {
        Ok(()) => println!("Screenshot saved to {}", output.display()),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run_command(cmd: Commands, socket: &PathBuf) {
    match cmd {
        Commands::Dump => cmd_dump(socket),
        Commands::Input { field, value } => cmd_input(socket, &field, &value),
        Commands::Click { label } => cmd_click(socket, &label),
        Commands::Submit => cmd_submit(socket),
        Commands::Key { key } => cmd_key(socket, &key),
        Commands::Ping => cmd_ping(socket),
        Commands::Screenshot { output } => cmd_screenshot(socket, &output),
        Commands::List => unreachable!(),
    }
}
