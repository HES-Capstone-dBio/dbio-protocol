{
  description = "Development environment for dBio protocol layer.";

  inputs = {
    utils.url = github:numtide/flake-utils;
    devshell.url = github:numtide/devshell;
    npmlock2nix.url = github:nix-community/npmlock2nix;
    npmlock2nix.flake = false;
  };

  outputs = { self, nixpkgs, utils, devshell, npmlock2nix }:
    utils.lib.eachDefaultSystem (system:

      let
        sbt-derivation = import (builtins.fetchTarball {
          url = "https://github.com/zaninime/sbt-derivation/archive/1ef212261cf7ad878c253192a1c8171de4d75b6d.tar.gz";
          sha256 = "1mz2s4hajc9cnrfs26d99ap4gswcidxcq441hg3aplnrmzrxbqbp";
        });

        aquaSrc = pkgs.fetchFromGitHub {
          owner = "fluencelabs";
          repo = "aqua";
          rev = "2fa3a095484e361d4ed66f160aaf49b804f20671";
          sha256 = "sha256-mX1uijIPwuI4rxwX5mHbXRlmyOLk8O9TOm6p5rrOk40=";
        };

        npm = pkgs.callPackage npmlock2nix { };

        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            devshell.overlay
            sbt-derivation
          ];
        };

        aqua = npm.build {
          pname = "aqua";
          version = "0.0.0";
          src = aquaSrc + "/npm";
          buildPhase = ''
            cp -r npm $out
            cp aqua.js $out/npm
            curl -o aqua.js https://github.com/fluencelabs/aqua/releases/download/0.6.3/aqua-0.6.3-282.js
          '';
          installPhase = ''
            cp cli/.js/target/scala-3.1.0/cli-opt/main.js ./npm/aqua.js
            mkdir -p $out/npm/dist
            mkdir $out/bin
            cp -r npm/* $out/npm/
            cd $out/npm
            npm install .
            npm run build
          '';
        };

      in rec {
        defaultPackage = aqua;
        devShell = pkgs.devshell.mkShell {
          name = "dBio-protocol";
          packages = with pkgs; [ cargo nodejs-16_x aqua ];
          commands = [];
        };
      }
  );
}
