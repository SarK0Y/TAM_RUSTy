#!/bin/bash
python3 /home/evg/Rust/TAM/ext/cargo.toml.set_ver.py -toml "/home/evg/Rust/TAM/ext/goto/Cargo.toml" -crate-name rst_lex -toml "/home/evg/Rust/TAM/ext/goto/Cargo.toml" -end
python3 /home/evg/Rust/TAM/ext/cargo.toml.set_ver.py -toml "/home/evg/Rust/TAM/ext/rst_lex/Cargo.toml"
python3 /home/evg/Rust/TAM/ext/cargo.toml.set_ver.py -toml "/home/evg/Rust/TAM/main/Cargo.toml" -crate-name goto1717 -toml "/home/evg/Rust/TAM/main/Cargo.toml"
python3 /home/evg/Rust/TAM/ext/cargo.toml.set_ver.py -toml "/home/evg/Rust/TAM/main/Cargo.toml" -crate-name rst_lex
