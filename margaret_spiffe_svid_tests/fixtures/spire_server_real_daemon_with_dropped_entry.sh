#!/usr/bin/env bash
# Reproduces a real production scenario: the spire-server daemon runs
# normally and `entry create` reports success, but the registration entry
# is never persisted (e.g. it is written to a different trust domain, or
# pruned immediately). The agent still attests and serves its Workload API
# socket, yet no entry ever matches the workload, so the Workload API never
# issues an identity. The SPIRE crates must surface this as a startup
# failure rather than hanging forever on an identity that will never arrive.
if [ "$1" = "entry" ]; then
    exit 0
fi
exec spire-server "$@"
