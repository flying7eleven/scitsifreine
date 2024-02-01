from argparse import ArgumentParser
from os import environ
from subprocess import Popen, PIPE

from scitsifreine import internal
from scitsifreine.exceptions import InsideTmuxSession, TmuxCommunicationError
from scitsifreine.internal import AnsibleInventory


class TmuxSession(object):
    def __init__(self, hosts: [str], close_on_exit: bool = False):
        if len(hosts) < 2:
            raise ValueError('You must specify at least two hosts to connect to')
        if self.__has_open_tmux_session():
            raise InsideTmuxSession('Cannot run script inside of an existing tmux session')
        self._hosts = hosts
        self._close_on_exit = close_on_exit
        self._session_name = internal.generate_session_name(host_list=self._hosts)
        self.__create_tmux_session()
        self.__create_split_panes()
        self.__open_ssh_connections()
        self.__attach_session()

    def __del__(self):
        if hasattr(self, '_close_on_exit') and self._close_on_exit:
            with (Popen(f'tmux kill-session -t {self._session_name}', shell=True, stdout=PIPE) as tmux):
                exit_code = tmux.wait()
                if 0 != exit_code:
                    raise TmuxCommunicationError(f'Failed to close a new tmux session. The exit code was {exit_code}')

    def __str__(self):
        return f'TmuxSession(session_name=\'{self._session_name}\')'

    @staticmethod
    def __has_open_tmux_session():
        return 'TMUX' in environ.keys()

    @staticmethod
    def __execute_command(command: str):
        with Popen(command, shell=True, stdout=PIPE, stderr=PIPE) as exec_command:
            exit_code = exec_command.wait()
            if 0 != exit_code:
                raise ChildProcessError(
                    f'Failed to run a command. The exit code was {exit_code} and its output to stderr "{exec_command.stderr.read().decode("utf")}"')

    def __create_tmux_session(self):
        # create a new session names by the used server (TODO: better naming with a large server list)
        TmuxSession.__execute_command(f'tmux new-session -d -s {self._session_name}')

        # rename the first window of the session to reflect the purpose
        TmuxSession.__execute_command('tmux rename-window -t 0 ssh-sessions')

    def __create_split_panes(self):
        v_splits = pane_idx = 0
        vertical_splits_remaining, horizontal_splits_remaining = internal.calculate_split_panes(self._hosts)
        while vertical_splits_remaining > 0 and v_splits < 2:
            TmuxSession.__execute_command('tmux split-window -v')
            vertical_splits_remaining -= 1
        while horizontal_splits_remaining > 0:
            TmuxSession.__execute_command(f'tmux select-pane -t {pane_idx}')
            TmuxSession.__execute_command('tmux split-window -h')
            pane_idx += 2
            horizontal_splits_remaining -= 1

    def __open_ssh_connections(self):
        for pane_idx in range(0, len(self._hosts)):
            current_host = self._hosts[pane_idx]
            TmuxSession.__execute_command(f'tmux select-pane -t {pane_idx} -T {current_host}')
            TmuxSession.__execute_command(f'tmux send-keys -t {pane_idx} "ssh {current_host}" C-m')

    def __attach_session(self):
        TmuxSession.__execute_command(f'tmux attach-session -t {self._session_name}:0')


def main_cli() -> int:
    from importlib.metadata import version

    # specify the arguments the user can pass
    arg_parser = ArgumentParser(prog='scitsi',
                                description='helper script for creating multiple ssh sessions using tmux',
                                epilog='see more details at: https://docs.tmux.org')
    arg_parser.add_argument('hosts', metavar='hosts', type=str, nargs='+', help='a list of hosts to connect to')
    arg_parser.add_argument('-c', '--close-on-exit', action='store_true',
                            help='terminate the tmux session when its closed or move to the background (detached)')
    arg_parser.add_argument('--version', action='version', version=f'%(prog)s {version("scitsifreine")}')
    arg_parser.add_argument('-l', '--ansible-host-lookup', action='store_true',
                            help='if set, the supplied hosts will be interpreted as environment and host group. '
                                 'Those will be used and looked up in the ansible inventory and look up the hosts of '
                                 'the host group for connecting')
    args = arg_parser.parse_args()
    used_hosts = args.hosts

    # if we are in ansible host group lookup, ensure that we have exactly two parameters provided
    if args.ansible_host_lookup:
        if len(args.hosts) != 2:
            print('Please provide the environment and host group to use an ansible inventory for host grouo lookup')
            return -1

        # read the inventory and try to figure out which hosts are the ones in the provided host group
        ansible_inventory = AnsibleInventory(environment=args.hosts[0])
        used_hosts = ansible_inventory.get_hosts(args.hosts[1])

    TmuxSession(hosts=used_hosts, close_on_exit=args.close_on_exit)
    return 0
