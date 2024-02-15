use log::error;
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

// TODO: this
#[derive(Eq, PartialEq, Debug)]
enum TmuxExecutionErrors {
    CreateSession,
    RenameWindow,
    VerticalSplit,
    HorizontalSplit,
    SelectPane,
    CalculateSplittingForHosts,
    SendKeysToSession,
}

/// The class for managing the tmux session.
pub struct Tmux<'a> {
    /// The hosts the instance currently manages.
    hosts: Vec<&'a str>,
    /// The name of the session the class instance is managing.
    session_name: String,
    /// Should be connections be closed on closing or detaching the tmux session?
    _close_on_exit: bool,
}

impl<'a> Tmux<'a> {
    /// Create a new instance of this struct.
    pub fn new(hosts: Vec<&str>, close_on_exit: bool) -> Tmux {
        Tmux {
            hosts,
            session_name: "".to_string(),
            _close_on_exit: close_on_exit,
        }
    }

    /// Check if the tmux command is available on the computer.
    pub fn is_tmux_available() -> bool {
        Tmux::execute_tmux_command(vec!["server-info"])
    }

    /// TODO
    pub fn open(&self) -> bool {
        self.generate_session_name("multissh");
        if let Err(error) = self.create_tmux_session_and_window() {
            match error {
                TmuxExecutionErrors::CreateSession => error!(
                    "Could not create new tmux session with the session name {}",
                    self.session_name
                ),
                TmuxExecutionErrors::RenameWindow => error!(
                    "Could not create and rename a window in the tmux session with the name {}",
                    self.session_name
                ),
                _ => error!("Unexpected error occurred while creating a new tmux session"),
            }
            return false;
        }
        if let Err(error) = self.create_split_panes() {
            match error {
                TmuxExecutionErrors::VerticalSplit => error!("Could not create a vertical split in the tmux session {}", self.session_name),
                TmuxExecutionErrors::HorizontalSplit => error!("Could not create a horizontal split in the tmux session {}", self.session_name),
                TmuxExecutionErrors::SelectPane => error!("Could not select a pane in the tmux session {}", self.session_name),
                TmuxExecutionErrors::CalculateSplittingForHosts => println!("Could not calculate the required splittings for the list of supplied {} hosts", self.hosts.len()),
                _ =>  error!("Unexpected error occurred while creating the required panes for the ssh connection to all hosts")
            }
            return false;
        }
        if let Err(error) = self.open_ssh_connections() {
            match error {
                TmuxExecutionErrors::SelectPane => error!(
                    "Failed to select a pane of the tmux session {}",
                    self.session_name
                ),
                TmuxExecutionErrors::SendKeysToSession => error!(
                    "Failed to send keys to a pane of the tmux session {}",
                    self.session_name
                ),
                _ => error!("Unexpected error occurred while opening the ssh connection to a host"),
            }
            return false;
        }
        self.attach_session();
        true
    }

    /// Execute any tmux command and return if the command succeeded or not.
    fn execute_tmux_command(arguments: Vec<&str>) -> bool {
        if let Ok(_process) = Command::new("tmux").args(arguments).spawn() {
            return true;
        }
        false
    }

    /// TODO
    fn create_tmux_session_and_window(&self) -> Result<(), TmuxExecutionErrors> {
        // create a new session names by the used server (TODO: better naming with a large server list)
        if !Tmux::execute_tmux_command(vec!["new-session", "-d", "-s", self.session_name.as_str()])
        {
            return Err(TmuxExecutionErrors::CreateSession);
        }

        // rename the first window of the session to reflect the purpose
        if !Tmux::execute_tmux_command(vec!["rename-window", "-t", "0", "ssh-sessions"]) {
            return Err(TmuxExecutionErrors::RenameWindow);
        }

        // it seems that everything was okay
        Ok(())
    }

    /// TODO
    fn create_split_panes(&self) -> Result<(), TmuxExecutionErrors> {
        if let Ok(splits) = self.calculate_split_panes_for_hosts() {
            let v_splits = 0;
            let mut pane_idx = 0;
            let mut vertical_splits_remaining = splits.vertical_split_count;
            let mut horizontal_splits_remaining = splits.horizontal_split_count;

            while vertical_splits_remaining > 0 && v_splits < 2 {
                if !Tmux::execute_tmux_command(vec!["split-window", "-v"]) {
                    return Err(TmuxExecutionErrors::VerticalSplit);
                }
                vertical_splits_remaining -= 1;
            }

            while horizontal_splits_remaining > 0 {
                if !Tmux::execute_tmux_command(vec![
                    "select-pane",
                    "-t",
                    pane_idx.to_string().as_str(),
                ]) {
                    return Err(TmuxExecutionErrors::SelectPane);
                }
                if !Tmux::execute_tmux_command(vec!["split-window", "-h"]) {
                    return Err(TmuxExecutionErrors::HorizontalSplit);
                }
                pane_idx += 2;
                horizontal_splits_remaining -= 1;
            }

            return Ok(());
        }
        Err(TmuxExecutionErrors::CalculateSplittingForHosts)
    }

    /// TODO
    fn open_ssh_connections(&self) -> Result<(), TmuxExecutionErrors> {
        let pane_idx = 0;
        for current_host in &self.hosts {
            if !Tmux::execute_tmux_command(vec![
                "select-pane",
                "-t",
                pane_idx.to_string().as_str(),
                "-T",
                current_host,
            ]) {
                return Err(TmuxExecutionErrors::SelectPane);
            }
            if !Tmux::execute_tmux_command(vec![
                "send-keys",
                "-t",
                pane_idx.to_string().as_str(),
                format!("\"ssh {current_host}\"").as_str(),
                "C-m",
            ]) {
                return Err(TmuxExecutionErrors::SendKeysToSession);
            }
        }
        Ok(())
    }

    fn attach_session(&self) {
        Tmux::execute_tmux_command(vec![
            "attach-session",
            "-t",
            format!("{0}:0", self.session_name).as_str(),
        ]);
    }

    /// Calculates how many horizontal and vertical splits are required to represent all ssh connections.
    fn calculate_split_panes_for_hosts(&self) -> Result<Splits, ScitsifreineErrors> {
        if self.hosts.is_empty() {
            return Err(ScitsifreineErrors::NoHosts);
        }
        let horizontal = (self.hosts.len() as f32 / 2.).ceil() - 1.;
        let vertical = (self.hosts.len() as f32 / 2.).floor();
        Ok(Splits::new(horizontal as u8, vertical as u8))
    }

    /// Generate a valid session name based on the hosts we should connect to.
    fn generate_session_name(&self, prefix: &str) -> String {
        let mut session_name = format!("{prefix}-");
        if !self.hosts.is_empty() {
            for current_host in &self.hosts {
                let host_part: Vec<&str> = current_host.split('.').collect();
                session_name.push_str(host_part[0]);
                session_name.push('-');
            }
        }
        session_name[..session_name.len() - 1].to_string()
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
