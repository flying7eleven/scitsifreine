def test_generate_session_name_with_empty_hosts():
    from scitsifreine.internal import generate_session_name
    assert 'multissh' == generate_session_name([])


def test_generate_session_name_with_none_as_hosts():
    from scitsifreine.internal import generate_session_name
    assert 'multissh' == generate_session_name(None)


def test_generate_session_name_with_simple_hosts_names():
    from scitsifreine.internal import generate_session_name
    assert 'multissh-first-second-third' == generate_session_name(['first', 'second', 'third'])


def test_generate_session_name_with_fqdn_hosts_names():
    from scitsifreine.internal import generate_session_name
    assert 'multissh-first-second-third' == generate_session_name(
        ['first.example.com', 'second.example.com', 'third.example.com'])


def test_number_of_splits_for_no_hosts():
    from scitsifreine.internal import calculate_split_panes
    vertical_splits_remaining, horizontal_splits_remaining = calculate_split_panes([])
    assert 0 == vertical_splits_remaining
    assert 0 == horizontal_splits_remaining


def test_number_of_splits_for_one_hosts():
    from scitsifreine.internal import calculate_split_panes
    vertical_splits_remaining, horizontal_splits_remaining = calculate_split_panes(['first'])
    assert 0 == vertical_splits_remaining
    assert 0 == horizontal_splits_remaining


def test_number_of_splits_for_two_hosts():
    from scitsifreine.internal import calculate_split_panes
    vertical_splits_remaining, horizontal_splits_remaining = calculate_split_panes(['first', 'second'])
    assert 0 == vertical_splits_remaining
    assert 1 == horizontal_splits_remaining


def test_number_of_splits_for_three_hosts():
    from scitsifreine.internal import calculate_split_panes
    vertical_splits_remaining, horizontal_splits_remaining = calculate_split_panes(
        ['first', 'second', 'third'])
    assert 1 == vertical_splits_remaining
    assert 1 == horizontal_splits_remaining


def test_number_of_splits_for_seven_hosts():
    from scitsifreine.internal import calculate_split_panes
    vertical_splits_remaining, horizontal_splits_remaining = calculate_split_panes(
        ['first', 'second', 'third', 'fourth', 'fifth', 'sixth', 'seventh'])
    assert 3 == vertical_splits_remaining
    assert 3 == horizontal_splits_remaining


def test_number_of_splits_for_ten_hosts():
    from scitsifreine.internal import calculate_split_panes
    vertical_splits_remaining, horizontal_splits_remaining = calculate_split_panes(
        ['first', 'second', 'third', 'fourth', 'fifth', 'sixth', 'seventh', 'eighth', 'ninth', 'tenth'])
    assert 4 == vertical_splits_remaining
    assert 5 == horizontal_splits_remaining


def test_number_of_splits_for_eleven_hosts():
    from scitsifreine.internal import calculate_split_panes
    vertical_splits_remaining, horizontal_splits_remaining = calculate_split_panes(
        ['first', 'second', 'third', 'fourth', 'fifth', 'sixth', 'seventh', 'eighth', 'ninth', 'tenth', 'eleventh'])
    assert 5 == vertical_splits_remaining
    assert 5 == horizontal_splits_remaining
