set shell := ["bash", "-euo", "pipefail", "-c"]

default:
	just --list

nix-checks:
	nix flake check
	system="$(nix eval --impure --raw --expr builtins.currentSystem)" && printf 'building current-system flake checks for %s\n' "$system" && nix build --print-build-logs ".#checks.$system.source-size" ".#checks.$system.release-metadata" ".#checks.$system.unsigned-release-artifact"

ci-fast:
	python3 scripts/check-source-size.py
	just nix-checks
	cargo test --workspace

ci: ci-fast
