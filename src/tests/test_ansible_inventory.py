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
    inventory_obj = AnsibleInventory(inventory_file)
    assert inventory_obj.is_group_known('some_name')
    assert not inventory_obj.is_group_known('some_name_invalid')
    assert inventory_obj.get_hosts('some_name') == ['host01.example.com', 'host02.example.com']
    assert not inventory_obj.get_hosts('some_name_invalid')


def test_read_simple_yaml_inventory(datadir):
    from scitsifreine.internal import AnsibleInventory
    inventory_file = datadir.join('test_inventory01.yml').strpath
    inventory_obj = AnsibleInventory(inventory_file)
    assert inventory_obj.is_group_known('some_name')
    assert not inventory_obj.is_group_known('some_name_invalid')
    assert inventory_obj.get_hosts('some_name') == ['host01.example.com', 'host02.example.com']
    assert not inventory_obj.get_hosts('some_name_invalid')


def test_read_ini_inventory_with_children(datadir):
    from scitsifreine.internal import AnsibleInventory
    inventory_file = datadir.join('test_inventory02.ini').strpath
    inventory_obj = AnsibleInventory(inventory_file)
    assert inventory_obj.is_group_known('some_name')
    assert not inventory_obj.is_group_known('some_name_invalid')
    assert inventory_obj.get_hosts('some_name') == ['host01.example.com', 'host02.example.com']
    assert not inventory_obj.get_hosts('some_name_invalid')


def test_read_yaml_inventory_with_children(datadir):
    from scitsifreine.internal import AnsibleInventory
    inventory_file = datadir.join('test_inventory02.yml').strpath
    inventory_obj = AnsibleInventory(inventory_file)
    assert inventory_obj.is_group_known('some_name')
    assert not inventory_obj.is_group_known('some_name_invalid')
    assert inventory_obj.get_hosts('some_name') == ['host01.example.com', 'host02.example.com']
    assert not inventory_obj.get_hosts('some_name_invalid')


def test_read_ini_inventory_with_additional_information(datadir):
    from scitsifreine.internal import AnsibleInventory
    inventory_file = datadir.join('test_inventory03.ini').strpath
    inventory_obj = AnsibleInventory(inventory_file)
    assert inventory_obj.is_group_known('some_name')
    assert not inventory_obj.is_group_known('some_name_invalid')
    assert inventory_obj.get_hosts('some_name') == ['host01.example.com', 'host02.example.com']
    assert not inventory_obj.get_hosts('some_name_invalid')


def test_read_yaml_inventory_with_additional_information(datadir):
    from scitsifreine.internal import AnsibleInventory
    inventory_file = datadir.join('test_inventory03.yml').strpath
    inventory_obj = AnsibleInventory(inventory_file)
    assert inventory_obj.is_group_known('some_name')
    assert not inventory_obj.is_group_known('some_name_invalid')
    assert inventory_obj.get_hosts('some_name') == ['host01.example.com', 'host02.example.com']
    assert not inventory_obj.get_hosts('some_name_invalid')
