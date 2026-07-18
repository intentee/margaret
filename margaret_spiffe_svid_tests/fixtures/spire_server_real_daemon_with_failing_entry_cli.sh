#!/usr/bin/env bash
# Reproduces a real production scenario: the spire-server daemon runs
# normally and earlier CLI subcommands succeed, but `entry create`
# rejects the request (e.g. duplicate registration, server-side
# validation failure). The SPIRE crates must surface this as a startup
# failure rather than continuing into an unregistered workload state.
if [ "$1" = "run" ]; then
    exec spire-server "$@"
fi
if [ "$1" = "entry" ]; then
    echo "scenario: entry create CLI rejected" >&2
    exit 1
fi
exec spire-server "$@"
