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
			pkgs = import nixpkgs { system = "x86_64-linux"; };
		in
		{
			devShells.x86_64-linux.default = pkgs.mkShell {
				buildInputs = [
					pkgs.pkgsCross.avr.buildPackages.gcc
					(pkgs.python3.withPackages (python-pkgs: with python-pkgs; [ pyserial ]))
					pkgs.minicom
					pkgs.ravedude
					(fenix.packages.x86_64-linux.fromToolchainFile {
						file = ./rust-toolchain.toml;
						sha256 = "sha256-z8J/GH7znPPg9kKvPirKcBeXqHikj1M7KB+anwsDx0M=";
					})
				];
				RAVEDUDE_PORT = "/dev/ttyACM0";
				AVR_HAL_BUILD_TARGETS = "all";

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
