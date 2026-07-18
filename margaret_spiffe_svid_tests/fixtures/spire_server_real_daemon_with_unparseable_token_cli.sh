#!/usr/bin/env bash
# Reproduces a real production scenario: the spire-server daemon runs
# normally, but the `token generate` CLI subcommand returns success with
# output our parser doesn't understand (e.g. a SPIRE version skew or a
# locale-translated label). The SPIRE crates must surface this as a
# parse error rather than hanging or panicking.
if [ "$1" = "run" ]; then
    exec spire-server "$@"
fi
if [ "$1" = "token" ]; then
    echo "scenario: unparseable token output with no Token: prefix"
    exit 0
fi
exec spire-server "$@"
