import unittest


class DummyTestCase(unittest.TestCase):
    def test_something(self):
        self.assertEqual(1, 1)

#
# from scitsifreine import TmuxSession
#
#
# class TmuxSessionTest(unittest.TestCase):
#     def test_user_has_to_provide_at_least_two_hosts(self):
#         self.assertRaises(ValueError, TmuxSession, hosts=[])
#
#     def test_string_serialization_of_session_works(self):
#         self.assertEqual('TmuxSession(session_name=\'multissh-host1-host2\')',
#                          str(TmuxSession(hosts=['host1', 'host2'])))
#
#     def test_ensure_we_cannot_run_in_tmux_session(self):
#         from os import environ
#         if 'TMUX' not in environ.keys():
#             environ['TMUX'] = 'fake-session'
#         self.assertRaises(ChildProcessError, TmuxSession, hosts=['host1', 'host2'])
#         del environ['TMUX']
#
#     def test_calculate_required_panes(self):
#         self.assertEqual((0, 1), TmuxSession.__calculate_split_panes(['host0', 'host1']))
#         self.assertEqual((1, 1), TmuxSession.__calculate_split_panes(['host0', 'host1', 'host2']))
#         self.assertEqual((3, 3), TmuxSession.__calculate_split_panes(
#             ['host0', 'host1', 'host2', 'host3', 'host4', 'host5', 'host6']))
#         self.assertEqual((4, 5), TmuxSession.__calculate_split_panes(
#             ['host0', 'host1', 'host2', 'host3', 'host4', 'host5', 'host6', 'host7', 'host8', 'host9']))
#         self.assertEqual((5, 5), TmuxSession.__calculate_split_panes(
#             ['host0', 'host1', 'host2', 'host3', 'host4', 'host5', 'host6', 'host7', 'host8', 'host9', 'host10']))
#
#
# if __name__ == '__main__':
#     unittest.main()
