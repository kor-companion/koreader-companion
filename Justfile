set shell := ["bash", "-euo", "pipefail", "-c"]

default:
	just --list

ci-fast:
	nix flake check

ci: ci-fast
