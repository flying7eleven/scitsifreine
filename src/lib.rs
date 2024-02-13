use std::process::Command;

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
pub struct Tmux<'a> {
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

    /// Check if the tmux command is available on the computer.
    pub fn is_tmux_available() -> bool {
        if let Ok(_process) = Command::new("tmux").arg("server-info").spawn() {
            return true;
        }
        false
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

    /// Generate a valid session name based on the hosts we should connect to.
    fn generate_session_name(self, prefix: &str) -> String {
        let mut session_name = format!("{prefix}-");
        if !self.hosts.is_empty() {
            for current_host in self.hosts {
                let host_part: Vec<&str> = current_host.split('.').collect();
                session_name.push_str(host_part[0]);
                session_name.push('-');
            }
        }
        return session_name[..session_name.len() - 1].to_string();
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

    #[test]
    fn test_generate_session_name_with_empty_hosts() {
        let session = Tmux::new(vec![], false);
        let session_name = session.generate_session_name("multissh");

        assert_eq!(session_name, "multissh");
    }

    #[test]
    fn test_generate_session_name_with_fqdn_hosts() {
        let session = Tmux::new(
            vec![
                "host1.example.com",
                "host2.example.com",
                "host3.example.com",
            ],
            false,
        );
        let session_name = session.generate_session_name("multissh");

        assert_eq!(session_name, "multissh-host1-host2-host3");
    }
}
