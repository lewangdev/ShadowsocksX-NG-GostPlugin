#!/usr/bin/env python3

import os


if __name__ == '__main__':
    ss_local_host = os.environ.get('SS_LOCAL_HOST')
    ss_local_port = os.environ.get('SS_LOCAL_PORT')
    ss_remote_host = os.environ.get('SS_REMOTE_HOST')
    ss_remote_port = os.environ.get('SS_REMOTE_PORT')
    ss_plugin_options = os.environ.get('SS_PLUGIN_OPTIONS')

    cmd = f"./gost/gost -L=ss://chacha20-ietf-poly1305:123456@{ss_local_host}:{ss_local_port} -F=wss://{ss_plugin_options}@{ss_remote_host}:{ss_remote_port}"
    os.system(cmd)
