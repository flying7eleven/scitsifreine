from math import floor, ceil
from ansible.inventory.manager import InventoryManager
from ansible.parsing.dataloader import DataLoader


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


def _get_correct_ansible_inventory(environment: str) -> str | None:
    pass


class AnsibleInventory(object):
    def __init__(self, environment: str | None = None, inventory_path: str | None = None):
        file_to_use = inventory_path if inventory_path else {
            _get_correct_ansible_inventory(environment)
        }
        if file_to_use is None:
            raise Exception()
        self._dl = DataLoader()
        self._im = InventoryManager(loader=self._dl, sources=file_to_use)

    def is_group_known(self, group_name: str) -> bool:
        return self._im.get_groups_dict().get(group_name) is not None

    def get_hosts(self, group_name: str) -> list[str] | None:
        if group_name in self._im.get_groups_dict().keys():
            return self._im.get_groups_dict()[group_name]
        else:
            return None
