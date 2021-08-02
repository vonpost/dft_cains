# dft_cains
A port of the DFT loot system in Rust+SQL https://www.reddit.com/r/classicwow/comments/j08phd/dft_fight_club_classictbc_simplified_loot_system/.

It's currently a newly started WIP. Idea is to migrate the entire sheet onto a SQL database from which all the calculations can be run much faster than in the Google Docs sheet.

The build environment is created with Nix:

shell.nix defines an environment which allows you to build the entire project so far (rust toolchain etc.). Just do ```nix-shell && cargo build```. 
In shell.nix you also define a environment variable ```DATABASE_URL``` which allows the sqlx to validate SQL queries during compile time. Additionally, container.nix creates a dummy developing SQL server that can be used for development purposes on NixOS by running ```nixos-container create foo --config-file container.nix```.
