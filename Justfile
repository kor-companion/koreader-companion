set shell := ["bash", "-euo", "pipefail", "-c"]

default:
	just --list

ci-fast:
	python3 scripts/check-source-size.py
	nix flake check
	cargo test --workspace

ci: ci-fast
