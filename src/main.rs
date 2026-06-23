use clap::{Parser, Subcommand};
#[cfg(not(test))]
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

#[cfg(not(test))]
fn main() {
    let cli = Cli::parse();
    let client = RealClient;

    if let Commands::List = cli.command {
        list_servers(&client);
        return;
    }

    let Some(socket) = resolve_socket(cli.socket, &client) else {
        return;
    };
    run_command(cli.command, &socket, &client);
}

trait DebugClient {
    fn find_servers(&self) -> Vec<PathBuf>;
    fn dump(&self, socket: &PathBuf) -> Result<String, String>;
    fn input(&self, socket: &PathBuf, field: &str, value: &str) -> Result<(), String>;
    fn click(&self, socket: &PathBuf, label: &str) -> Result<(), String>;
    fn submit(&self, socket: &PathBuf) -> Result<(), String>;
    fn key(&self, socket: &PathBuf, key: &str) -> Result<(), String>;
    fn ping(&self, socket: &PathBuf) -> Result<(), String>;
    fn screenshot_to_file(&self, socket: &PathBuf, output: &PathBuf) -> Result<(), String>;
}

#[cfg(not(test))]
struct RealClient;

#[cfg(not(test))]
impl DebugClient for RealClient {
    fn find_servers(&self) -> Vec<PathBuf> {
        client::find_servers()
    }

    fn dump(&self, socket: &PathBuf) -> Result<String, String> {
        client::dump(socket).map_err(|error| error.to_string())
    }

    fn input(&self, socket: &PathBuf, field: &str, value: &str) -> Result<(), String> {
        client::input(socket, field, value).map_err(|error| error.to_string())
    }

    fn click(&self, socket: &PathBuf, label: &str) -> Result<(), String> {
        client::click(socket, label).map_err(|error| error.to_string())
    }

    fn submit(&self, socket: &PathBuf) -> Result<(), String> {
        client::submit(socket).map_err(|error| error.to_string())
    }

    fn key(&self, socket: &PathBuf, key: &str) -> Result<(), String> {
        client::key(socket, key).map_err(|error| error.to_string())
    }

    fn ping(&self, socket: &PathBuf) -> Result<(), String> {
        client::ping(socket).map_err(|error| error.to_string())
    }

    fn screenshot_to_file(&self, socket: &PathBuf, output: &PathBuf) -> Result<(), String> {
        client::screenshot_to_file(socket, output).map_err(|error| error.to_string())
    }
}

fn resolve_socket(explicit: Option<PathBuf>, client: &impl DebugClient) -> Option<PathBuf> {
    if let Some(s) = explicit {
        return Some(s);
    }
    let servers = client.find_servers();
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

fn list_servers(client: &impl DebugClient) {
    let servers = client.find_servers();
    if servers.is_empty() {
        println!("No iced debug servers running");
    } else {
        for s in servers {
            println!("{}", s.display());
        }
    }
}

fn cmd_dump(client: &impl DebugClient, socket: &PathBuf) {
    match client.dump(socket) {
        Ok(layout) => println!("{}", layout),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_input(client: &impl DebugClient, socket: &PathBuf, field: &str, value: &str) {
    match client.input(socket, field, value) {
        Ok(()) => println!("OK"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_click(client: &impl DebugClient, socket: &PathBuf, label: &str) {
    match client.click(socket, label) {
        Ok(()) => println!("OK"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_submit(client: &impl DebugClient, socket: &PathBuf) {
    match client.submit(socket) {
        Ok(()) => println!("OK"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_key(client: &impl DebugClient, socket: &PathBuf, key: &str) {
    match client.key(socket, key) {
        Ok(()) => println!("Sent key '{}'", key),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_ping(client: &impl DebugClient, socket: &PathBuf) {
    match client.ping(socket) {
        Ok(()) => println!("Pong"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_screenshot(client: &impl DebugClient, socket: &PathBuf, output: &PathBuf) {
    match client.screenshot_to_file(socket, output) {
        Ok(()) => println!("Screenshot saved to {}", output.display()),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run_command(cmd: Commands, socket: &PathBuf, client: &impl DebugClient) {
    match cmd {
        Commands::Dump => cmd_dump(client, socket),
        Commands::Input { field, value } => cmd_input(client, socket, &field, &value),
        Commands::Click { label } => cmd_click(client, socket, &label),
        Commands::Submit => cmd_submit(client, socket),
        Commands::Key { key } => cmd_key(client, socket, &key),
        Commands::Ping => cmd_ping(client, socket),
        Commands::Screenshot { output } => cmd_screenshot(client, socket, &output),
        Commands::List => list_servers(client),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    struct FakeClient {
        servers: Vec<PathBuf>,
        calls: RefCell<Vec<String>>,
    }

    impl FakeClient {
        fn with_servers(servers: &[&str]) -> Self {
            Self {
                servers: servers.iter().map(PathBuf::from).collect(),
                calls: RefCell::new(Vec::new()),
            }
        }

        fn calls(&self) -> Vec<String> {
            self.calls.borrow().clone()
        }
    }

    impl DebugClient for FakeClient {
        fn find_servers(&self) -> Vec<PathBuf> {
            self.calls.borrow_mut().push("find_servers".to_string());
            self.servers.clone()
        }

        fn dump(&self, socket: &PathBuf) -> Result<String, String> {
            self.calls
                .borrow_mut()
                .push(format!("dump:{}", socket.display()));
            Ok("layout".to_string())
        }

        fn input(&self, socket: &PathBuf, field: &str, value: &str) -> Result<(), String> {
            self.calls
                .borrow_mut()
                .push(format!("input:{}:{field}:{value}", socket.display()));
            Ok(())
        }

        fn click(&self, socket: &PathBuf, label: &str) -> Result<(), String> {
            self.calls
                .borrow_mut()
                .push(format!("click:{}:{label}", socket.display()));
            Ok(())
        }

        fn submit(&self, socket: &PathBuf) -> Result<(), String> {
            self.calls
                .borrow_mut()
                .push(format!("submit:{}", socket.display()));
            Ok(())
        }

        fn key(&self, socket: &PathBuf, key: &str) -> Result<(), String> {
            self.calls
                .borrow_mut()
                .push(format!("key:{}:{key}", socket.display()));
            Ok(())
        }

        fn ping(&self, socket: &PathBuf) -> Result<(), String> {
            self.calls
                .borrow_mut()
                .push(format!("ping:{}", socket.display()));
            Ok(())
        }

        fn screenshot_to_file(&self, socket: &PathBuf, output: &PathBuf) -> Result<(), String> {
            self.calls.borrow_mut().push(format!(
                "screenshot:{}:{}",
                socket.display(),
                output.display()
            ));
            Ok(())
        }
    }

    #[test]
    fn parses_cli_commands() {
        assert!(matches!(
            Cli::try_parse_from(["iced-debug", "dump"]).unwrap().command,
            Commands::Dump
        ));
        assert!(matches!(
            Cli::try_parse_from(["iced-debug", "--socket", "/tmp/app.sock", "input", "Email", "a@b"])
                .unwrap()
                .command,
            Commands::Input { field, value } if field == "Email" && value == "a@b"
        ));
        assert!(matches!(
            Cli::try_parse_from(["iced-debug", "screenshot", "out.jpg"])
                .unwrap()
                .command,
            Commands::Screenshot { output } if output == PathBuf::from("out.jpg")
        ));
    }

    #[test]
    fn resolve_socket_handles_explicit_zero_one_and_many_servers() {
        assert_eq!(
            resolve_socket(
                Some(PathBuf::from("/tmp/explicit.sock")),
                &FakeClient::default()
            ),
            Some(PathBuf::from("/tmp/explicit.sock"))
        );
        assert_eq!(resolve_socket(None, &FakeClient::default()), None);

        let one = FakeClient::with_servers(&["/tmp/one.sock"]);
        assert_eq!(
            resolve_socket(None, &one),
            Some(PathBuf::from("/tmp/one.sock"))
        );

        let many = FakeClient::with_servers(&["/tmp/one.sock", "/tmp/two.sock"]);
        assert_eq!(resolve_socket(None, &many), None);
    }

    #[test]
    fn list_servers_handles_empty_and_nonempty_sets() {
        let empty = FakeClient::default();
        list_servers(&empty);
        assert_eq!(empty.calls(), vec!["find_servers"]);

        let one = FakeClient::with_servers(&["/tmp/one.sock"]);
        list_servers(&one);
        assert_eq!(one.calls(), vec!["find_servers"]);
    }

    #[test]
    fn run_command_dispatches_all_client_operations() {
        let client = FakeClient::default();
        let socket = PathBuf::from("/tmp/app.sock");

        run_command(Commands::Dump, &socket, &client);
        run_command(
            Commands::Input {
                field: "Email".to_string(),
                value: "a@b".to_string(),
            },
            &socket,
            &client,
        );
        run_command(
            Commands::Click {
                label: "Login".to_string(),
            },
            &socket,
            &client,
        );
        run_command(Commands::Submit, &socket, &client);
        run_command(
            Commands::Key {
                key: "Escape".to_string(),
            },
            &socket,
            &client,
        );
        run_command(Commands::Ping, &socket, &client);
        run_command(
            Commands::Screenshot {
                output: PathBuf::from("out.jpg"),
            },
            &socket,
            &client,
        );

        assert_eq!(
            client.calls(),
            vec![
                "dump:/tmp/app.sock",
                "input:/tmp/app.sock:Email:a@b",
                "click:/tmp/app.sock:Login",
                "submit:/tmp/app.sock",
                "key:/tmp/app.sock:Escape",
                "ping:/tmp/app.sock",
                "screenshot:/tmp/app.sock:out.jpg",
            ]
        );
    }
}
