from math import ceil, floor
from os import environ
from subprocess import Popen, PIPE
from argparse import ArgumentParser


class TmuxSession(object):
    def __init__(self, hosts: [str]):
        if len(hosts) < 2:
            raise ValueError('You must specify at least two hosts to connect to')
        if self._has_open_tmux_session():
            raise ChildProcessError('Cannot run script inside of an existing tmux session')
        self._hosts = hosts
        self._session_name = self._generate_session_name(host_list=self._hosts)
        self._create_tmux_session()
        self._create_split_panes()
        self._open_ssh_connections()
        self._attach_session()

    def __del__(self):
        with Popen(f'tmux kill-session -t {self._session_name}', shell=True, stdout=PIPE) as tmux:
            exit_code = tmux.wait()
            if 0 != exit_code:
                raise ChildProcessError(f'Failed to close a new tmux session. The exit code was {exit_code}')

    def __str__(self):
        return f'TmuxSession(session_name=\'{self._session_name}\')'

    @staticmethod
    def _generate_session_name(host_list: [str], prefix='multissh'):
        session_name = f'{prefix}-'
        for current_host in host_list:
            session_name += f'{current_host.split(".")[0]}-'
        return session_name[:-1]

    @staticmethod
    def _has_open_tmux_session():
        return 'TMUX' in environ.keys()

    @staticmethod
    def _calculate_split_panes(host_list: [str]) -> (int, int):
        return ceil(len(host_list) / 2.) - 1, floor(len(host_list) / 2.)

    @staticmethod
    def _execute_command(command: str):
        with Popen(command, shell=True, stdout=PIPE, stderr=PIPE) as exec_command:
            exit_code = exec_command.wait()
            if 0 != exit_code:
                raise ChildProcessError(
                    f'Failed to run a command. The exit code was {exit_code} and its output to stderr "{exec_command.stderr.read().decode("utf")}"')

    def _create_tmux_session(self):
        # create a new session names by the used server (TODO: better naming with a large server list)
        TmuxSession._execute_command(f'tmux new-session -d -s {self._session_name}')

        # rename the first window of the session to reflect the purpose
        TmuxSession._execute_command(f'tmux rename-window -t 0 ssh-sessions')

    def _create_split_panes(self):
        v_splits = pane_idx = 0
        vertical_splits_remaining, horizontal_splits_remaining = TmuxSession._calculate_split_panes(self._hosts)
        while vertical_splits_remaining > 0 and v_splits < 2:
            TmuxSession._execute_command(f'tmux split-window -v')
            vertical_splits_remaining -= 1
        while horizontal_splits_remaining > 0:
            TmuxSession._execute_command(f'tmux select-pane -t {pane_idx}')
            TmuxSession._execute_command(f'tmux split-window -h')
            pane_idx += 2
            horizontal_splits_remaining -= 1

    def _open_ssh_connections(self):
        for pane_idx in range(0, len(self._hosts)):
            current_host = self._hosts[pane_idx]
            print(current_host)
            TmuxSession._execute_command(f'tmux select-pane -t {pane_idx} -T {current_host}')
            TmuxSession._execute_command(f'tmux send-keys -t {pane_idx} "ssh {current_host}" C-m')

    def _attach_session(self):
        TmuxSession._execute_command(f'tmux attach-session -t {self._session_name}:0')


def main():
    arg_parser = ArgumentParser(prog='scitsifréine',
                                description='Helper script for creating multiple ssh sessions using tmux',
                                epilog='See more details at: https://docs.tmux.org')
    arg_parser.add_argument('hosts', metavar='hosts', type=str, nargs='+', help='a list of hosts to connect to')
    args = arg_parser.parse_args()
    TmuxSession(hosts=args.hosts)


if __name__ == '__main__':
    main()
