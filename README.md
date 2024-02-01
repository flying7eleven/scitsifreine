# Scitsifréine (Irish for _schizophrenia_)
[![Lint and test Python package](https://github.com/flying7eleven/scitsifreine/actions/workflows/python-lint-test.yml/badge.svg)](https://github.com/flying7eleven/scitsifreine/actions/workflows/python-lint-test.yml)
[![MIT License](http://img.shields.io/badge/license-MIT-9370d8.svg?style=flat)](http://opensource.org/licenses/MIT)

Scitsifréine, or short `scitsi` is a small tool for using [tmux](https://github.com/tmux/tmux/wiki) for connecting to multiple remote servers via [ssh](https://en.wikipedia.org/wiki/Secure_Shell).

## Using ansible inventories for host lookup
In most cases, the hosts you want to connect to are defined in a central registry or ansible inventory file.
To use those sources for a direct host lookup, you have to define the `SCITSIFREINE_ANSIBLE_INVENTORIES` environment variable with the inventory files you want to use.

```shell
$ export SCITSIFREINE_ANSIBLE_INVENTORIES="/path/to/an/inventory"
```

If you want to use different files for a host lookup (e.g. if you split the inventory into a development and a production environment) you can do this as well:

```shell
$ export SCITSIFREINE_ANSIBLE_INVENTORIES="development=/path/to/an/inventory_dev,production=development=/path/to/an/inventory_live"
```

After this environment variable is set, you can uae the following syntax to connect to a corresponding host group:

```shell
$ scitsi --ansible-host-lookup <environment> <host_group>
```