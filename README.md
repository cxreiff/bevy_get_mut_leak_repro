# bevy_get_mut_leak_repro

Minimal reproduction to show a potential Bevy memory leak.

1. `cargo run`
2. Memory usage should be stable.
3. Press and hold the space key, running the update system.
4. As you hold the key, memory usage should steadily increase.
5. Release the key. The system is no longer running, but the memory usage does not decrease.

