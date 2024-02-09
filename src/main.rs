use clap::{Parser, Subcommand};

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

fn main() {
    // parse the supplied arguments
    let _arguments = Args::parse();
}
