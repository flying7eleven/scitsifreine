from __future__ import unicode_literals
from shutil import copytree

from pytest import fixture
import os


@fixture
def datadir(tmpdir, request):
    """
    Fixture responsible for searching a folder with the same name of test
    module and, if available, moving all contents to a temporary directory so
    tests can use them freely.
    """
    filename = request.module.__file__
    test_dir, _ = os.path.splitext(filename)

    if os.path.isdir(test_dir):
        copytree(test_dir, str(tmpdir), dirs_exist_ok=True)

    return tmpdir


def test_read_simple_ini_inventory(datadir):
    from scitsifreine.internal import AnsibleInventory
    inventory_file = datadir.join('test_inventory01.ini').strpath
    inventory_obj = AnsibleInventory(inventory_path=inventory_file)
    assert inventory_obj.is_group_known('some_name')
    assert not inventory_obj.is_group_known('some_name_invalid')
    assert inventory_obj.get_hosts('some_name') == ['host01.example.com', 'host02.example.com']
    assert not inventory_obj.get_hosts('some_name_invalid')


def test_read_simple_yaml_inventory(datadir):
    from scitsifreine.internal import AnsibleInventory
    inventory_file = datadir.join('test_inventory01.yml').strpath
    inventory_obj = AnsibleInventory(inventory_path=inventory_file)
    assert inventory_obj.is_group_known('some_name')
    assert not inventory_obj.is_group_known('some_name_invalid')
    assert inventory_obj.get_hosts('some_name') == ['host01.example.com', 'host02.example.com']
    assert not inventory_obj.get_hosts('some_name_invalid')


def test_read_ini_inventory_with_children(datadir):
    from scitsifreine.internal import AnsibleInventory
    inventory_file = datadir.join('test_inventory02.ini').strpath
    inventory_obj = AnsibleInventory(inventory_path=inventory_file)
    assert inventory_obj.is_group_known('some_name')
    assert not inventory_obj.is_group_known('some_name_invalid')
    assert inventory_obj.get_hosts('some_name') == ['host01.example.com', 'host02.example.com']
    assert not inventory_obj.get_hosts('some_name_invalid')


def test_read_yaml_inventory_with_children(datadir):
    from scitsifreine.internal import AnsibleInventory
    inventory_file = datadir.join('test_inventory02.yml').strpath
    inventory_obj = AnsibleInventory(inventory_path=inventory_file)
    assert inventory_obj.is_group_known('some_name')
    assert not inventory_obj.is_group_known('some_name_invalid')
    assert inventory_obj.get_hosts('some_name') == ['host01.example.com', 'host02.example.com']
    assert not inventory_obj.get_hosts('some_name_invalid')


def test_read_ini_inventory_with_additional_information(datadir):
    from scitsifreine.internal import AnsibleInventory
    inventory_file = datadir.join('test_inventory03.ini').strpath
    inventory_obj = AnsibleInventory(inventory_path=inventory_file)
    assert inventory_obj.is_group_known('some_name')
    assert not inventory_obj.is_group_known('some_name_invalid')
    assert inventory_obj.get_hosts('some_name') == ['host01.example.com', 'host02.example.com']
    assert not inventory_obj.get_hosts('some_name_invalid')


def test_read_yaml_inventory_with_additional_information(datadir):
    from scitsifreine.internal import AnsibleInventory
    inventory_file = datadir.join('test_inventory03.yml').strpath
    inventory_obj = AnsibleInventory(inventory_path=inventory_file)
    assert inventory_obj.is_group_known('some_name')
    assert not inventory_obj.is_group_known('some_name_invalid')
    assert inventory_obj.get_hosts('some_name') == ['host01.example.com', 'host02.example.com']
    assert not inventory_obj.get_hosts('some_name_invalid')


def test_none_will_be_returned_if_env_is_not_defined():
    from scitsifreine.internal import get_correct_ansible_inventory
    inventory_file = get_correct_ansible_inventory('some_name')
    assert not inventory_file


def test_inventory_will_be_returned_if_not_associated_with_env():
    from scitsifreine.internal import get_correct_ansible_inventory
    from os import environ
    environ['SCITSIFREINE_ANSIBLE_INVENTORIES'] = '/tmp/inventory'
    inventory_file1 = get_correct_ansible_inventory('some_name')
    inventory_file2 = get_correct_ansible_inventory('some_other_name')
    assert inventory_file1 == '/tmp/inventory'
    assert inventory_file2 == '/tmp/inventory'


def test_correct_inventory_will_be_returned_if_multiple_environments_are_defined():
    from scitsifreine.internal import get_correct_ansible_inventory
    from os import environ
    environ['SCITSIFREINE_ANSIBLE_INVENTORIES'] = 'live=/tmp/inventory1,production=/tmp/inventory2'
    inventory_file1 = get_correct_ansible_inventory('live')
    inventory_file2 = get_correct_ansible_inventory('not_defined')
    assert inventory_file1 == '/tmp/inventory1'
    assert not inventory_file2
