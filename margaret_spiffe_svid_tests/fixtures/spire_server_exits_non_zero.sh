#!/usr/bin/env bash
# Reproduces a real spire-server CLI failure: the binary writes a SPIRE-style
# error to stderr and exits with a non-zero status code. The SPIRE crates
# must remain stable when bundle show / token generate / entry create are
# rejected (e.g. server unavailable, permission denied, malformed args).
echo "spire-server: command failed" >&2
exit 1
