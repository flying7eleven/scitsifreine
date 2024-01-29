import unittest


class InternalMethods(unittest.TestCase):
    def test_generate_session_name_with_empty_hosts(self):
        from scitsifreine.internal import generate_session_name
        self.assertEqual('multissh', generate_session_name([]))

    def test_generate_session_name_with_none_as_hosts(self):
        from scitsifreine.internal import generate_session_name
        self.assertEqual('multissh', generate_session_name(None))

    def test_generate_session_name_with_simple_hosts_names(self):
        from scitsifreine.internal import generate_session_name
        self.assertEqual('multissh-first-second-third', generate_session_name(['first', 'second', 'third']))

    def test_generate_session_name_with_fqdn_hosts_names(self):
        from scitsifreine.internal import generate_session_name
        self.assertEqual('multissh-first-second-third',
                         generate_session_name(['first.example.com', 'second.example.com', 'third.example.com']))

    def test_number_of_splits_for_no_hosts(self):
        from scitsifreine.internal import calculate_split_panes
        vertical_splits_remaining, horizontal_splits_remaining = calculate_split_panes([])
        self.assertEqual(0, vertical_splits_remaining)
        self.assertEqual(0, horizontal_splits_remaining)

    def test_number_of_splits_for_one_hosts(self):
        from scitsifreine.internal import calculate_split_panes
        vertical_splits_remaining, horizontal_splits_remaining = calculate_split_panes(['first'])
        self.assertEqual(0, vertical_splits_remaining)
        self.assertEqual(0, horizontal_splits_remaining)

    def test_number_of_splits_for_two_hosts(self):
        from scitsifreine.internal import calculate_split_panes
        vertical_splits_remaining, horizontal_splits_remaining = calculate_split_panes(['first', 'second'])
        self.assertEqual(0, vertical_splits_remaining)
        self.assertEqual(1, horizontal_splits_remaining)

    def test_number_of_splits_for_three_hosts(self):
        from scitsifreine.internal import calculate_split_panes
        vertical_splits_remaining, horizontal_splits_remaining = calculate_split_panes(
            ['first', 'second', 'third'])
        self.assertEqual(1, vertical_splits_remaining)
        self.assertEqual(1, horizontal_splits_remaining)

    def test_number_of_splits_for_seven_hosts(self):
        from scitsifreine.internal import calculate_split_panes
        vertical_splits_remaining, horizontal_splits_remaining = calculate_split_panes(
            ['first', 'second', 'third', 'fourth', 'fifth', 'sixth', 'seventh'])
        self.assertEqual(3, vertical_splits_remaining)
        self.assertEqual(3, horizontal_splits_remaining)

    def test_number_of_splits_for_ten_hosts(self):
        from scitsifreine.internal import calculate_split_panes
        vertical_splits_remaining, horizontal_splits_remaining = calculate_split_panes(
            ['first', 'second', 'third', 'fourth', 'fifth', 'sixth', 'seventh', 'eighth', 'ninth', 'tenth'])
        self.assertEqual(4, vertical_splits_remaining)
        self.assertEqual(5, horizontal_splits_remaining)

    def test_number_of_splits_for_eleven_hosts(self):
        from scitsifreine.internal import calculate_split_panes
        vertical_splits_remaining, horizontal_splits_remaining = calculate_split_panes(
            ['first', 'second', 'third', 'fourth', 'fifth', 'sixth', 'seventh', 'eighth', 'ninth', 'tenth', 'eleventh'])
        self.assertEqual(5, vertical_splits_remaining)
        self.assertEqual(5, horizontal_splits_remaining)
