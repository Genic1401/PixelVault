# pixelvault-optical

Initial Rust/WASM-compatible transport core: bounded PVOT framing with BLAKE3 validation and a simple XOR reconstruction helper. The `raptorq` dependency is included for the planned fountain-code phase, but the current code does not yet implement RaptorQ packetization, camera modulation, or decoding. Use visible, user-consented optical patterns and do not rely on this prototype for secure transport.
