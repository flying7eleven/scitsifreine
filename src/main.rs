use clap::{Parser, Subcommand};
use scitsifreine::Tmux;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The algorithm which should be used for hashing.
    #[clap(subcommand)]
    connection_mode: ConnectionModes,
    /// Close the ssh connection if the tmux session gets closed or detached.
    #[arg(short, long, default_value_t = false)]
    close_on_exit: bool,
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

fn connection_mode_direct(close_on_exit: bool, hosts: Vec<&str>) {
    let tmux_connection = Tmux::new(hosts, close_on_exit);
    tmux_connection.open();
}

fn connection_mode_ansible(close_on_exit: bool, _environment: &str, _host_group: &str) {
    let _tmux_connection = Tmux::new(vec![], close_on_exit);
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
