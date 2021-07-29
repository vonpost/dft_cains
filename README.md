# dft_cains
A port of the DFT loot system in Rust+SQL https://www.reddit.com/r/classicwow/comments/j08phd/dft_fight_club_classictbc_simplified_loot_system/.

It's currently a newly started WIP. Idea is to migrate the entire sheet onto a SQL database from which all the calculations can be run much faster than in the Google Docs sheet.

It has Nix support, shell.nix defines an environment which allows you to build the entire project so far. Just do nix-shell and then cargo build. 
