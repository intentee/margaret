#!/usr/bin/env bash
# Reproduces a real spire-server version-skew scenario: the binary returns
# zero but stdout is not the JSON join-token document our parser expects.
# The SPIRE crates must surface this as a parse error rather than hanging or
# panicking.
echo "unexpected payload that does not look like a join token"
exit 0
