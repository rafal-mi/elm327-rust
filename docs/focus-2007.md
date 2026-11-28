## Focus 2007 discovery replay

Run `cargo run -- --profile focus-2007` or `cargo run -- --rfcomm --profile focus-2007` (Linux Bluetooth setup/SDP still required). Send CR-terminated commands. The two bitmap payloads are captured from the 2007 Focus 1.8 TDCi; protocol A6 and CAN ID 7E8 are synthetic assumptions because headers were disabled. NO DATA for accelerator requests is simulated, not captured. Other live values and DTCs are not implemented. Run `cargo test` (existing hardware tests require Bluetooth) or `cargo test captured_bitmaps_and_header_mode` and `cargo test stream_replay`.
