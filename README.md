# Egui Encrypted Text Editor

An [Egui](https://crates.io/crates/egui) app to read and write to encrypted files.

IMPORTANT: The key used for encryption/decryption is *the same key everytime*! Currently there is no way to change the key.

Encryption is very slow on debug build, but fast on release build.
