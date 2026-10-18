# AetherDCC Studio v1.0

`aether-dcc-sdk` is a zero-dependency Windows Rust package for non-binary silicon compute. One workspace holds the quaternary contention ALU, the Intel UHD (`0x9B41`) 1.58-bit digit network, the GhostVault SIMD cipher, the Kleene/Priest paradox isolator, and a 64-node Ising solver. The same crate exports a thread-safe C ABI, an interactive Win32 studio, and a release scorecard.

The digit glyphs are geometric block codes on an 8×8 canvas. They are not a handwritten MNIST font.

## Requirements

- Windows x86_64, Rust 2021 (tested with rustc/cargo 1.97).
- An Intel iGPU that enumerates as vendor `0x8086`, device `0x9B41`, feature level 11.1. The studio and the GPU tests refuse any other adapter. WARP does not pass.
- Logical CPUs 0, 2, 4, and 6 free for the ALU. On the i5-10210U those are the four physical cores. The process raises its priority class. The main thread is not pinned.

The crate links `kernel32`, `user32`, `gdi32`, `d3d11`, `dxgi`, and `d3dcompiler_47` by raw FFI. `[dependencies]` is empty.

## Install

```text
git clone https://github.com/ramuu09/aether-dcc-sdk.git
cd aether-dcc-sdk
cargo build --release
```

Release builds use `opt-level = 3` and `lto = false`.

## Quickstart

Open the studio:

```text
cargo run --release
```

The window title is `AetherDCC Studio v1.0 — Non-Binary Silicon Compute & 1.58-Bit AI Product`. Draw on the 8×8 canvas with the left button (`+1`) and the right button (`−1`). Keys `0`–`9` stamp a code glyph, `N` flips 8 pixels, `G` refreshes GhostVault, and `C` clears the canvas to −1. If Windows reports no monitor, the process exits with status 2.

Print the product scorecard without opening a window:

```text
cargo run --release -- --product-benchmark
```

Run the QA suite on one thread. The CPU pool and the D3D device are process-wide, so parallel test threads deadlock.

```text
cargo test --release -- --test-threads=1 --nocapture
```

Call the digit net from Rust:

```rust
use aether_dcc_sdk::{argmax, glyph_f32, GpuEngine};

fn main() {
    let mut gpu = GpuEngine::new();
    let sample = gpu.bitnet(&glyph_f32(3));
    println!("class {} in {:.1} us", argmax(sample.scores), sample.dispatch_us);
}
```

`argmax` returns `-1` when every class score ties, including the all-zero blank canvas.

## C ABI

`cargo build --release` produces `target/release/aether_dcc_sdk.dll`. Exports are `extern "C"`, `#[no_mangle]`, and serialized by one process mutex. Each call builds the engine it needs and drops it before returning. A null pointer returns `-1`.

| Export | Role |
| --- | --- |
| `dcc_sdk_version` | Pointer to `"AetherDCC Studio v1.0"`. |
| `dcc_sdk_infer_digit10` | 64 pixels in, class and 10 scores out. Returns `0` when the GPU matches the host reference. |
| `dcc_sdk_ghost_cipher` | 16-byte block, key, and salt. Cipher is `(block XOR key) + salt` per byte. Returns the bit-error count. |
| `dcc_sdk_eval_script` | `0` tax total, `1` cipher round-trip, `2` hybrid accumulator. |
| `dcc_sdk_solve_ising64` | 64 nodes, 8 neighbor slots each. Writes spins and energy. |

```c
const char *dcc_sdk_version(void);
int dcc_sdk_infer_digit10(const float *pixels, int *class_out, int *scores, double *latency_us);
int dcc_sdk_ghost_cipher(const unsigned char *block, const unsigned char *key,
                         const unsigned char *salt, unsigned char *out, double *latency_us);
int dcc_sdk_eval_script(int script_id, int *result);
int dcc_sdk_solve_ising64(const int *neighbors, const int *coupling, const int *bias,
                          int *spins_out, int *energy_out, double *latency_us);
```

On the release host, script `0` returned `622`, script `1` returned `0`, and script `2` returned `136`.

## Architecture

- **Quaternary ALU.** One sensor thread and three synapse threads run `lock xadd` on a 4096-byte `VirtualAlloc` page. They are unpinned, sit at below-normal priority, and wait on a condition variable between jobs. Each channel is 256 bytes. The positive bank sits at the start of the channel and the negative bank sits 64 bytes later. Pedestal 4 and burst 4 produce the plateaus −0.600, −0.200, +0.200, and +0.600, read as magnitudes −3, −1, +1, and +3. The ratio is `Y = (t_pos − t_neg) / (t_pos + t_neg)`. A 0/0 measurement is +0.000, never NaN. Instructions are ADD, SUB, AND, OR, XOR, CMP, and JNZ (the CMP zero flag).
- **One-sided preemption ablation.** Three estimators read the same five symbols. The harness injects one synthetic stall of 5,000,000 ticks onto the negative rail of group 0 in each of 500 trials. It does not spawn burn threads. `NaiveRailSumEstimator` adds every group into the two rails. `ChopperPreemptionGatedEstimator` drops a group whose energy exceeds 3.5 times the clean-group baseline and takes the median of the remaining per-group ratios. `WorkConservingTurnEstimator` is one parked ALU probe per symbol: the counter advances on a completed `lock xadd`, so the synthetic stall is absent from `Y`. The measured table is in the benchmark section. The proof is in [WHITE_PAPER.md](WHITE_PAPER.md).
- **DCC-BitNet.** One `cs_5_0` dispatch, 64 threads, maps 64 pixels through 16 ternary hidden neurons to 10 digit classes. A zero weight skips `InterlockedAdd`. An empty canvas (every pixel −1) scores +0 on all ten classes and is reported as `BLANK / NO_SIGNAL`.
- **GhostVault.** `VEC_XOR16` then `VEC_ADD16` on the UHD compute shader. Steady-state block latency is measured in the fuzzer, separately from the cold-start ABI call that also creates the device.
- **Paradox and Ising.** An odd liar ring settles in one ALU pass at `Y = +0.000`. The same GPU runs a 64-node, degree-8 Ising relaxation. Energy is the sum of `s_i * s_j` over edges. A pass requires GPU energy at or below the binary greedy energy.

The theory and the drift-cancellation proof are in [WHITE_PAPER.md](WHITE_PAPER.md).

## Benchmark

Host: Intel i5-10210U, DXGI adapter `Intel(R) UHD Graphics`, vendor `0x8086`, device `0x9B41`, feature level 11.1. The cool table is the warm `cargo run --release -- --product-benchmark` on 2026-10-18 (command wall clock 0.88 s, exit 0). Latencies are microseconds. The retired table under it is the earlier long hostile campaign on the same date. That campaign is not what the command prints now.

| Scenario | Ops/Cycles | Latency p50 / p99 µs | Errors | Status |
| --- | ---: | ---: | ---: | --- |
| Cool sentinel pass | 189 | 379.0 / 13,618.4 | 1 | PASS |
| Endurance, 32 cycles | 32 | 39.9 / 1,663.8 | 0 | PASS |
| CPU + UHD collision | 8 | 497.6 / 2,247.7 | 0 | PASS |
| Adversarial fuzzer, 32 ops | 49 | 40.5 / 2,234.0 | 0 | PASS |
| Vision noise cliff | 10 | 1,025.9 / 1,722.5 | 0 | PASS |
| Paradox and Ising graphs | 221 | 70.3 / 1,752.2 | 0 | PASS |

Six of six scenarios passed. The one error on the sentinel row is a class miss (`Nominal quiet, auth on` read `SPOOFED_TELEMETRY_ANOMALY`). Bit errors on that row were 0. The pass rule allows at most one class miss. The fuzzer compared 1,367 bits at 0.000000% BER. Endurance kept the four plateaus, full-scale counts 12/0, 0/12, and 4/4, with 0 deadlocks and 0 overflows. BitNet matched the host (p50 438.0 µs, p99 1,257.2 µs): 10/10 clean glyphs, and 1/1 at 12 flips. `cargo test --release -- --test-threads=1` finished 9 passed, 0 failed, in 0.60 s once `shader-cache/` held the Ising bytecode. The process that compiled and wrote that cache finished in 40.55 s. With the studio window open and idle, the process accumulated 0.000 ms of processor time across an 800 ms sample.

### One-sided preemption ablation

`cargo run --release -- --product-benchmark` prints this table after the six scenarios. The cool process printed `product pillars bitnet PASS stress PASS ablation PASS`, `elapsed 0.823 ms`, and `CPU burn threads 0`. Each of 500 trials receives one 5,000,000-tick stall on the negative rail of group 0. The turn row is five parked ALU probes. Plateau centers are the median \(Y\) of each symbol. The naive medians sit near −1 because the stall is on every trial. The −3 class still decodes correctly, which is why the symbol-error rate is 80% and not 100%.

| Estimator | Median Y (−3, −1, 0, +1, +3) | Symbol errors |
| :--- | :---: | :---: |
| Naive cumulative rail-sum | −0.997, −0.994, −0.997, −0.990, −0.987 | 400 / 500 (80.000000%, BER 40.000000%) |
| Chopper + denominator gate | −0.600, −0.200, +0.000, +0.200, +0.600 | 0 / 500 (0.000000%, BER 0.000000%) |
| Work-conserving turn pedestal | −0.600, −0.200, +0.000, +0.200, +0.600 | 0 / 500 (0.000000%, BER 0.000000%) |

The same binary's release test `test_pillar6_preemption_ablation_naive_vs_hardened` printed the same three rates, elapsed 0.996 ms.

Retired live windows, same date, before the thermal patch. They are not the cells above. The product-benchmark window of that binary printed naive 1,239 / 10,000 (12.390%, BER 8.030000%) and 0 / 10,000 on the chopper and the turn pedestal, at CPU busy 100.0%. Its release test printed naive 2,304 / 10,000 (23.040%, BER 12.826667%) and again 0 / 10,000 on the other two rows, and the suite finished in 235.70 s. An earlier sum of every micro-slice duration, outside this crate, measured 4,266 / 10,000 (42.660%).

Retired scorecard from the long hostile release-report process:

| Scenario | Ops/Cycles | Latency p50 / p99 µs | Errors | Status |
| --- | ---: | ---: | ---: | --- |
| Hostile OS storm | 1,181 | 5.5 / 2,028.7 | 0 | PASS |
| Endurance marathon | 10,000 | 2.4 / 23.3 | 0 | PASS |
| CPU + UHD collision | 200 | 206.2 / 972.9 | 0 | PASS |
| Adversarial ALU fuzzer | 50,513 | 5.1 / 22.2 | 0 | PASS |
| Vision noise cliff | 322 | 941.3 / 1,181.8 | 0 | PASS |
| Paradox and Ising graphs | 221 | 1.1 / 2,063.2 | 0 | PASS |

## Layout

```text
aether-dcc-sdk/
├── Cargo.toml
├── Cargo.lock
├── LICENSE
├── README.md
├── WHITE_PAPER.md
└── src/
    ├── lib.rs
    ├── main.rs
    ├── estimator_ablation.rs
    ├── studio.rs
    └── shaders/
        ├── bitnet_digit.hlsl
        └── ghost_vault.hlsl
```

## License

MIT. Copyright (c) 2026 Agaran. See [LICENSE](LICENSE).
