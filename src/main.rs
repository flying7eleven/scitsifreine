use ascii_table::{Align, AsciiTable};
use clap::{Parser, Subcommand};
use log::{debug, LevelFilter};
use scitsifreine::Tmux;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::process::{Command as ProcessCommand, Stdio};
use std::str;

/// A struct for combining information about the path of an executable
/// with the corresponding version information of that tool.
struct ApplicationTuple {
    /// The path to the binary for which the information are provided for.
    binary: Option<String>,
    /// The version information for the given binary.
    version: Option<String>,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The algorithm which should be used for hashing.
    #[clap(subcommand)]
    command: Commands,
    /// If the flag is used, the tmux connection will not be automatically attached to the screen.
    #[arg(short, long, default_value_t = false, conflicts_with = "close_on_exit")]
    no_auto_attach: bool,
    /// Close the ssh connection if the tmux session gets closed or detached.
    #[arg(
        short,
        long,
        default_value_t = false,
        conflicts_with = "no_auto_attach"
    )]
    close_on_exit: bool,
    /// Log everything to a logfile in the same directory the program is executed in.
    #[arg(short, long, default_value_t = false)]
    logging: bool,
    /// Log all calls down to the trace category (requires to also set the flag for logging).
    #[arg(short, long, default_value_t = false, requires = "logging")]
    trace_logging: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Use Ansible inventories to look up the hosts to connect to by supplying host groups to the tool.
    Ansible {
        /// The environment for which the host group should be looked up (e.g. dev, prod, etc.).
        environment: String,
        /// The host group to connect to (e.g. host_group_1, etc.).
        host_group: String,
    },

    /// Use the hostnames supplied on the command line for connecting to the remote hosts.
    Direct {
        /// The hosts the new multi-ssh session should connect to.
        #[arg(required = true, num_args = 2..)]
        hosts: Vec<String>,
    },

    /// Show all available information available to scitsifreine.
    Information,
}

#[cfg(debug_assertions)]
#[inline(always)]
fn logging_level(trace_logging: bool) -> LevelFilter {
    if trace_logging {
        return LevelFilter::Trace;
    }
    LevelFilter::Debug
}

#[cfg(not(debug_assertions))]
#[inline(always)]
fn logging_level(trace_logging: bool) -> LevelFilter {
    if trace_logging {
        return LevelFilter::Trace;
    }
    LevelFilter::Info
}

fn setup_logging(trace_logging: bool) {
    use chrono::Utc;

    // create an instance for the Dispatcher to create a new logging configuration
    let mut base_config = fern::Dispatch::new();

    // set the corresponding logging level
    base_config = base_config.level(logging_level(trace_logging));

    // create the logfile we want to use for logging
    let maybe_logfile = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .append(false)
        .open("scitsifreine.log");
    if maybe_logfile.is_err() {
        panic!("Could not create logfile. Don't know how to recover from that.");
    }

    // define how a logging line should look like and attach the streams to which the output will be
    // written to
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                Utc::now().format("[%H:%M:%S]"),
                record.level(),
                message
            ))
        })
        .chain(maybe_logfile.unwrap());

    // now chain everything together and get ready for actually logging stuff
    base_config.chain(file_config).apply().unwrap();
}

fn get_application_information(command: &str, version_flag: &str) -> ApplicationTuple {
    let mut cmd = ProcessCommand::new("which");
    cmd.arg(command)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());
    if let Ok(status) = cmd.status() {
        if status.success() {
            let binary_path = String::from_utf8_lossy(cmd.output().unwrap().stdout.as_ref())
                .trim()
                .to_string();
            cmd = ProcessCommand::new(command);
            cmd.arg(version_flag)
                .stderr(Stdio::piped())
                .stdout(Stdio::piped());
            if let Ok(status) = cmd.status() {
                if status.success() {
                    let stdout_output =
                        String::from_utf8_lossy(cmd.output().unwrap().stdout.as_ref())
                            .trim()
                            .to_string();
                    return if !stdout_output.is_empty() {
                        ApplicationTuple {
                            binary: Some(binary_path),
                            version: Some(stdout_output),
                        }
                    } else {
                        let stderr_output =
                            String::from_utf8_lossy(cmd.output().unwrap().stderr.as_ref())
                                .trim()
                                .to_string();
                        ApplicationTuple {
                            binary: Some(binary_path),
                            version: Some(stderr_output),
                        }
                    };
                }
            }
            return ApplicationTuple {
                binary: Some(binary_path),
                version: None,
            };
        }
    }
    ApplicationTuple {
        binary: None,
        version: None,
    }
}

fn connection_mode_direct(close_on_exit: bool, auto_attach: bool, hosts: Vec<&str>) {
    debug!(
        "Using direct connection mode to connect to {} hosts",
        hosts.len()
    );
    let tmux_connection = Tmux::new(hosts, close_on_exit, auto_attach);
    tmux_connection.open();
}

fn connection_mode_ansible(
    _close_on_exit: bool,
    _auto_attach: bool,
    environment: &str,
    host_group: &str,
) {
    debug!(
        "Using ansible-inventory connection mode for environment {} and host group {}",
        environment, host_group
    );
    unimplemented!("This connection mode is currently not implemented")
}

fn show_information() {
    let mut ascii_table = AsciiTable::default();
    ascii_table
        .column(0)
        .set_header("Application")
        .set_align(Align::Left);
    ascii_table
        .column(1)
        .set_header("Path")
        .set_align(Align::Left);
    ascii_table
        .column(2)
        .set_header("Version")
        .set_align(Align::Left);

    let tmux_infos = get_application_information("tmux", "-V");
    let tmux_binary = tmux_infos.binary.unwrap_or("N/A".to_string());
    let tmux_version = tmux_infos.version.unwrap_or("N/A".to_string());
    let ssh_infos = get_application_information("ssh", "-V");
    let ssh_binary = ssh_infos.binary.unwrap_or("N/A".to_string());
    let ssh_version = ssh_infos.version.unwrap_or("N/A".to_string());
    let data: Vec<Vec<&dyn Display>> = vec![
        vec![&"tmux", &tmux_binary, &tmux_version],
        vec![&"ssh", &ssh_binary, &ssh_version],
    ];
    ascii_table.print(data);
}

fn main() {
    // parse the supplied arguments
    let arguments = Args::parse();

    // if logging should be enabled, do it now
    if arguments.logging {
        setup_logging(arguments.trace_logging);
    }

    // based on the supplied mode, call the correct entrypoint
    match &arguments.command {
        Commands::Ansible {
            environment,
            host_group,
        } => {
            if !Tmux::is_tmux_available() {
                println!("Cannot find tmux. Please install it before using scitsifreine.");
                std::process::exit(1);
            }
            connection_mode_ansible(
                arguments.close_on_exit,
                !arguments.no_auto_attach,
                environment,
                host_group,
            )
        }
        Commands::Direct { hosts } => {
            if !Tmux::is_tmux_available() {
                println!("Cannot find tmux. Please install it before using scitsifreine.");
                std::process::exit(1);
            }
            connection_mode_direct(
                arguments.close_on_exit,
                !arguments.no_auto_attach,
                hosts.iter().map(|s| &**s).collect(),
            )
        }
        Commands::Information => show_information(),
    }
}
