let
  rust-version = "nightly";

  mozilla-overlay =
    import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);

  pkgs = import <nixpkgs> {
    overlays = [ mozilla-overlay ];
  };

  rust-channel = pkgs.rustChannelOf {
    channel = rust-version;
  };

  rust = rust-channel.rust.override {
    extensions = [ "rust-src" ];
  };

  cargo = rust-channel.cargo;
in
  pkgs.mkShell {
    name = "rust-dev";
    buildInputs = [pkgs.postgresql pkgs.openssl pkgs.pkg-config rust cargo pkgs.rustracer pkgs.cargo-edit ];
    DATABASE_URL = "postgres://10.233.1.2/foo";
  }
