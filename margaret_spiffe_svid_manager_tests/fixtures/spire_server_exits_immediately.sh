#!/usr/bin/env bash
# Reproduces a real SPIRE server crash during startup: the binary exits
# successfully without ever creating its socket. The SPIRE crates must
# remain stable when this happens (e.g. wait_until_socket_ready times
# out instead of hanging forever).
exit 0
