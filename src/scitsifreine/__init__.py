from math import ceil, floor
from os import environ
from subprocess import Popen, PIPE

from scitsifreine import internal
from scitsifreine.exceptions import InsideTmuxSession, TmuxCommunicationError


class TmuxSession(object):
    def __init__(self, hosts: [str]):
        if len(hosts) < 2:
            raise ValueError('You must specify at least two hosts to connect to')
        if self.__has_open_tmux_session():
            raise InsideTmuxSession('Cannot run script inside of an existing tmux session')
        self._hosts = hosts
        self._session_name = internal.generate_session_name(host_list=self._hosts)
        self.__create_tmux_session()
        self.__create_split_panes()
        self.__open_ssh_connections()
        self.__attach_session()

    def __del__(self):
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
    def __calculate_split_panes(host_list: [str]) -> (int, int):
        return ceil(len(host_list) / 2.) - 1, floor(len(host_list) / 2.)

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
        vertical_splits_remaining, horizontal_splits_remaining = TmuxSession.__calculate_split_panes(self._hosts)
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
            print(current_host)
            TmuxSession.__execute_command(f'tmux select-pane -t {pane_idx} -T {current_host}')
            TmuxSession.__execute_command(f'tmux send-keys -t {pane_idx} "ssh {current_host}" C-m')

    def __attach_session(self):
        TmuxSession.__execute_command(f'tmux attach-session -t {self._session_name}:0')
