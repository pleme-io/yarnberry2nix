
build:
	nix build .#yarnberry2nix

run:
	nix run .#yarnberry2nix

cargo2nix:
	nix run github:cargo2nix/cargo2nix
