{
  description = "Cera, editeur de texte terminal en Rust";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAll = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAll (pkgs: rec {
        cera = pkgs.rustPlatform.buildRustPackage {
          pname = "cera";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = [ pkgs.makeWrapper ];

          postInstall = ''
            mkdir -p $out/share/cera
            cp -r plugins $out/share/cera/plugins
            patchShebangs $out/share/cera/plugins
            wrapProgram $out/bin/cera --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.python3 ]}
          '';

          meta = {
            description = "Editeur de texte terminal avec systeme de plugins";
            homepage = "https://github.com/kelian-4/cera";
            mainProgram = "cera";
            license = pkgs.lib.licenses.gpl3Plus;
            platforms = pkgs.lib.platforms.linux;
          };
        };
        default = cera;
      });

      apps = forAll (pkgs: {
        default = {
          type = "app";
          program = "${self.packages.${pkgs.system}.cera}/bin/cera";
        };
      });

      devShells = forAll (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            clippy
            rustfmt
            rust-analyzer
            python3
            nodejs
            gcc
          ];
        };
      });
    };
}
