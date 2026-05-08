set shell := ["bash", "-euo", "pipefail", "-c"]

default:
	just --list

ci-fast:
	nix flake check
	cargo test --workspace

ci: ci-fast
