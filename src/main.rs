use clap::{Parser, Subcommand};
use log::{debug, LevelFilter};
use scitsifreine::Tmux;
use std::fs::OpenOptions;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The algorithm which should be used for hashing.
    #[clap(subcommand)]
    connection_mode: ConnectionModes,
    /// Close the ssh connection if the tmux session gets closed or detached.
    #[arg(short, long, default_value_t = false)]
    close_on_exit: bool,
    /// Log everything to a logfile in the same directory the program is executed in.
    #[arg(short, long, default_value_t = false)]
    logging: bool,
}

#[derive(Subcommand)]
enum ConnectionModes {
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
}

#[cfg(debug_assertions)]
#[inline(always)]
fn logging_level() -> LevelFilter {
    LevelFilter::Debug
}

#[cfg(not(debug_assertions))]
#[inline(always)]
fn logging_level() -> LevelFilter {
    LevelFilter::Info
}

fn setup_logging() {
    use chrono::Utc;

    // create an instance for the Dispatcher to create a new logging configuration
    let mut base_config = fern::Dispatch::new();

    // set the corresponding logging level
    base_config = base_config.level(logging_level());

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

fn connection_mode_direct(close_on_exit: bool, hosts: Vec<&str>) {
    debug!(
        "Using direct connection mode to connect to {} hosts",
        hosts.len()
    );
    let tmux_connection = Tmux::new(hosts, close_on_exit);
    tmux_connection.open();
}

fn connection_mode_ansible(_close_on_exit: bool, environment: &str, host_group: &str) {
    debug!(
        "Using ansible-inventory connection mode for environment {} and host group {}",
        environment, host_group
    );
    unimplemented!("This connection mode is currently not implemented")
}

fn main() {
    // if tmux cannot be found, we can exit early
    if !Tmux::is_tmux_available() {
        println!("Cannot find tmux. Please install it before using scitsifreine.");
        std::process::exit(1);
    }

    // parse the supplied arguments
    let arguments = Args::parse();

    // if logging should be enabled, do it now
    if arguments.logging {
        setup_logging();
    }

    // based on the supplied mode, call the correct entrypoint
    match &arguments.connection_mode {
        ConnectionModes::Ansible {
            environment,
            host_group,
        } => connection_mode_ansible(arguments.close_on_exit, environment, host_group),
        ConnectionModes::Direct { hosts } => connection_mode_direct(
            arguments.close_on_exit,
            hosts.iter().map(|s| &**s).collect(),
        ),
    }
}
