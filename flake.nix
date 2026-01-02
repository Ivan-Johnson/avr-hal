{
	description = "AVR HAL development environment";

	inputs = {
		nixpkgs.url = "nixpkgs/nixos-25.11-small";
		fenix = {
			url = "github:nix-community/fenix";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs =
		{
			self,
			nixpkgs,
			fenix,
		}:
		let
			pkgs = import nixpkgs {
				system = "x86_64-linux";
				config.allowUnfreePredicate = pkg: builtins.elem (nixpkgs.lib.getName pkg) [ "vscode" ];
			};
		in
		{
			devShells.x86_64-linux.default = pkgs.mkShell {
				buildInputs = [
					pkgs.pkgsCross.avr.buildPackages.gcc
					(pkgs.python3.withPackages (python-pkgs: with python-pkgs; [ pyserial ]))
					pkgs.minicom
					pkgs.ravedude
					pkgs.vscode
					(fenix.packages.x86_64-linux.fromToolchainFile {
						file = ./rust-toolchain.toml;

						# 20251230->present
						sha256 = "sha256-UTAqJO6LFvfLyZTO7d3myyE+rdMP/Mny0m0n/jBKzLQ=";
					})
				];
				RAVEDUDE_PORT = "/dev/ttyACM0";
				AVR_HAL_BUILD_TARGETS = "arduino-micro";
				# NIXPKGS_ALLOW_UNFREE=1;

				# Setting PATH directly doesn't work at all:
				#
				# PATH = ./devtools/bin;
				#
				# Even if it did work, it would probably copy everything into the nix store.
				# I don't want to have to re-run `nix develop` every time I modify one of
				# the devtools. As such, I go with this slightly uglier approach instead:
				shellHook = ''PATH="$(realpath devtools/bin/):$PATH";'';
			};
		};
}
