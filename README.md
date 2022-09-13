<img src="https://user-images.githubusercontent.com/11075892/189526526-4a0a0049-1714-4ba4-a967-3a7f62d5e5e0.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189526521-ac0f8644-4915-4feb-b371-d9b3add3cb44.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189526523-4ae3ff55-6758-4bf9-93b2-fbde7a4267f2.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189526525-7e69a486-a416-4fa9-a980-4e8fe9186bbc.svg" width="100px" />

# ColorID

[![Package version](https://img.shields.io/crates/v/colorid.svg)](https://crates.io/crates/colorid)
[![License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://github.com/rustq/colorid/blob/master/LICENSE)

`Color as Identify - The ColorID in Rust`

`一款基于颜色表示身份的 Rust 随机 ID 生成器`

## Install

```toml
[dependencies]
nanoid = "0.0.1"
```

## Usage

```rust
use colorid::colorid;

fn main() {
   let id = colorid!(); // #A3F68E-#33EFAF-#8CB823-#D82C91
}
```

## Benchmark

```rust
colorid                 time:   [843.68 ns 850.76 ns 858.04 ns]
uuid                    time:   [904.09 ns 922.00 ns 941.15 ns]
nanoid                  time:   [1.1065 µs 1.1211 µs 1.1371 µs]
```

[runs/8309729209](https://github.com/rustq/colorid/runs/8309729209)

## License

[MIT](https://opensource.org/licenses/MIT)


## ColorID in Other Languages

- [JavaScript](https://github.com/rustq/colorid.js)