from math import floor, ceil
from toml import load as tomlload


def generate_session_name(host_list: [str], prefix='multissh'):
    session_name = f'{prefix}-'
    if host_list:
        for current_host in host_list:
            session_name += f'{current_host.split(".")[0]}-'
    return session_name[:-1]


def calculate_split_panes(host_list: [str]) -> (int, int):
    if not host_list or len(host_list) == 0:
        return 0, 0
    return ceil(len(host_list) / 2.) - 1, floor(len(host_list) / 2.)


class AnsibleInventory(object):
    def __init__(self, inventory: str):
        with open(inventory, 'r') as f:
            self._inventory = tomlload(f)
