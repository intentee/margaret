#!/usr/bin/env bash
# Reproduces a real production scenario: the spire-server daemon runs
# normally, but the `bundle show` CLI subcommand rejects the request
# (e.g. server unavailable, permission denied). The SPIRE crates must
# surface this as a startup failure, not hang waiting for the bundle.
if [ "$1" = "run" ]; then
    exec spire-server "$@"
fi
if [ "$1" = "bundle" ]; then
    echo "scenario: bundle show CLI rejected" >&2
    exit 1
fi
exec spire-server "$@"
