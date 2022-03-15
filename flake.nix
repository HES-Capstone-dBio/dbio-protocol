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
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ devshell.overlay ];
        };

        npm = pkgs.callPackage npmlock2nix { };

        aquaSrc = pkgs.fetchFromGitHub {
          owner = "fluencelabs";
          repo = "aqua";
          rev = "2fa3a095484e361d4ed66f160aaf49b804f20671";
          sha256 = "sha256-mX1uijIPwuI4rxwX5mHbXRlmyOLk8O9TOm6p5rrOk40=";
        };

        # This derivation doesn't work for now, do not use
        aqua = npm.build {
          src = aquaSrc + "/npm";
          version = "0.0.1";
          unpackPhase = ''
            curl -o aqua.js https://github.com/fluencelabs/aqua/releases/download/0.6.3/aqua-0.6.3-282.js
          '';
          installPhase = ''
            cp -r dist $out
            cp aqua.js $out
          '';
          buildCommands = [ "npm run build" ];
        };

      in rec {
        defaultPackage = pkgs.hello;
        devShell = pkgs.devshell.mkShell {
          name = "dBio-protocol";
          packages = with pkgs; [ cargo nodejs-17_x ];
          commands = [];
        };
      }
  );
}
