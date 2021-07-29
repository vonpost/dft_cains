{ pkgs, ... }:

{

  networking.firewall.enable = false;
  documentation.nixos.enable = false;

  services.postgresql = {
    enable = true;
    enableTCPIP = true;
    authentication = "host all all 0.0.0.0/0 trust";
    initialScript = pkgs.writeText "backend-initScript" ''
      CREATE ROLE foo WITH LOGIN PASSWORD 'foo' CREATEDB;
      CREATE DATABASE foo;
      GRANT ALL PRIVILEGES ON DATABASE foo TO foo;
    '';
  };
}
