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

/// The number of required splits for the horizontal and vertical direction.
#[derive(Debug)]
struct Splits {
    horizontal_split_count: u8,
    vertical_split_count: u8,
}

impl Splits {
    pub fn new(horizontal_split_count: u8, vertical_split_count: u8) -> Splits {
        Splits {
            horizontal_split_count,
            vertical_split_count,
        }
    }
}

/// The possible errors which can happen during creating and managing a tmux session.
#[derive(Eq, PartialEq, Debug)]
enum ScitsifreineErrors {
    /// There were no hosts supplied directly on the command line or indirectly by an ansible inventory.
    NoHosts,
}

/// The class for managing the tmux session.
struct Tmux<'a> {
    /// The hosts the instance currently manages.
    hosts: Vec<&'a str>,
    /// Should be connections be closed on closing or detaching the tmux session?
    close_on_exit: bool,
}

impl<'a> Tmux<'a> {
    /// Create a new instance of this struct.
    pub fn new(hosts: Vec<&str>, close_on_exit: bool) -> Tmux {
        Tmux {
            hosts,
            close_on_exit,
        }
    }

    /// Calculates how many horizontal and vertical splits are required to represent all ssh connections.
    fn calculate_split_panes_for_hosts(self) -> Result<Splits, ScitsifreineErrors> {
        if self.hosts.is_empty() {
            return Err(ScitsifreineErrors::NoHosts);
        }
        let horizontal = (self.hosts.len() as f32 / 2.).ceil() - 1.;
        let vertical = (self.hosts.len() as f32 / 2.).floor();
        Ok(Splits::new(horizontal as u8, vertical as u8))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ScitsifreineErrors, Tmux};

    #[test]
    fn test_number_of_splits_for_no_hosts() {
        let session = Tmux::new(vec![], false);
        let maybe_split_result = session.calculate_split_panes_for_hosts();

        assert_eq!(maybe_split_result.is_err(), true);
        assert_eq!(
            *maybe_split_result.as_ref().unwrap_err(),
            ScitsifreineErrors::NoHosts
        );
    }

    #[test]
    fn test_number_of_splits_for_one_hosts() {
        let session = Tmux::new(vec!["first"], false);
        let maybe_split_result = session.calculate_split_panes_for_hosts();

        assert_eq!(maybe_split_result.is_err(), false);
        assert_eq!(
            maybe_split_result.as_ref().unwrap().horizontal_split_count,
            0
        );
        assert_eq!(maybe_split_result.as_ref().unwrap().vertical_split_count, 0);
    }

    #[test]
    fn test_number_of_splits_for_two_hosts() {
        let session = Tmux::new(vec!["first", "second"], false);
        let maybe_split_result = session.calculate_split_panes_for_hosts();

        assert_eq!(maybe_split_result.is_err(), false);
        assert_eq!(
            maybe_split_result.as_ref().unwrap().horizontal_split_count,
            0
        );
        assert_eq!(maybe_split_result.as_ref().unwrap().vertical_split_count, 1);
    }

    #[test]
    fn test_number_of_splits_for_three_hosts() {
        let session = Tmux::new(vec!["first", "second", "third"], false);
        let maybe_split_result = session.calculate_split_panes_for_hosts();

        assert_eq!(maybe_split_result.is_err(), false);
        assert_eq!(
            maybe_split_result.as_ref().unwrap().horizontal_split_count,
            1
        );
        assert_eq!(maybe_split_result.as_ref().unwrap().vertical_split_count, 1);
    }

    #[test]
    fn test_number_of_splits_for_seven_hosts() {
        let session = Tmux::new(
            vec![
                "first", "second", "third", "fourth", "fifth", "sixth", "seventh",
            ],
            false,
        );
        let maybe_split_result = session.calculate_split_panes_for_hosts();

        assert_eq!(maybe_split_result.is_err(), false);
        assert_eq!(
            maybe_split_result.as_ref().unwrap().horizontal_split_count,
            3
        );
        assert_eq!(maybe_split_result.as_ref().unwrap().vertical_split_count, 3);
    }

    #[test]
    fn test_number_of_splits_for_ten_hosts() {
        let session = Tmux::new(
            vec![
                "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eights",
                "ninth", "tenth",
            ],
            false,
        );
        let maybe_split_result = session.calculate_split_panes_for_hosts();

        assert_eq!(maybe_split_result.is_err(), false);
        assert_eq!(
            maybe_split_result.as_ref().unwrap().horizontal_split_count,
            4
        );
        assert_eq!(maybe_split_result.as_ref().unwrap().vertical_split_count, 5);
    }

    #[test]
    fn test_number_of_splits_for_eleven_hosts() {
        let session = Tmux::new(
            vec![
                "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eights",
                "ninth", "tenth", "eleventh",
            ],
            false,
        );
        let maybe_split_result = session.calculate_split_panes_for_hosts();

        assert_eq!(maybe_split_result.is_err(), false);
        assert_eq!(
            maybe_split_result.as_ref().unwrap().horizontal_split_count,
            5
        );
        assert_eq!(maybe_split_result.as_ref().unwrap().vertical_split_count, 5);
    }
}

fn connection_mode_direct(close_on_exit: bool, _hosts: &[String]) {
    let _tmux_connection = Tmux::new(vec![], close_on_exit);
    unimplemented!("This connection mode is currently not implemented")
}

fn connection_mode_ansible(close_on_exit: bool, _environment: &str, _host_group: &str) {
    let _tmux_connection = Tmux::new(vec![], close_on_exit);
    unimplemented!("This connection mode is currently not implemented")
}

fn main() {
    // parse the supplied arguments
    let arguments = Args::parse();

    // based on the supplied mode, call the correct entrypoint
    match &arguments.connection_mode {
        ConnectionModes::Ansible {
            environment,
            host_group,
        } => connection_mode_ansible(arguments.close_on_exit, environment, host_group),
        ConnectionModes::Direct { hosts } => connection_mode_direct(arguments.close_on_exit, hosts),
    }
}
