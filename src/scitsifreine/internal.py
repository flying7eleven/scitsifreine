def generate_session_name(host_list: [str], prefix='multissh'):
    session_name = f'{prefix}-'
    for current_host in host_list:
        session_name += f'{current_host.split(".")[0]}-'
    return session_name[:-1]
