import unittest


class AnsibleInventoryTest(unittest.TestCase):
    def test_generate_session_name_with_empty_hosts(self):
        from scitsifreine.internal import generate_session_name
        self.assertEqual('multissh', generate_session_name([]))
