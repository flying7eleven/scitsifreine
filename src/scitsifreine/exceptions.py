class InsideTmuxSession(Exception):
    """
    The InsideTmuxSession exception will be raised when the code detects, that it runs inside another
    tmux session.
    """
    pass


class TmuxCommunicationError(Exception):
    """
    The TmuxCommunicationError exception will be raised if any communication with the tmux process failed.
    """
    pass


class NoInventoryFileSpecified(Exception):
    """
    The NoInventoryFileSpecified exception will be raised if neither an inventory file or an environment was specified.
    """
    pass
