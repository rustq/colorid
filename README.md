<img src="https://user-images.githubusercontent.com/11075892/189526526-4a0a0049-1714-4ba4-a967-3a7f62d5e5e0.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189526521-ac0f8644-4915-4feb-b371-d9b3add3cb44.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189526523-4ae3ff55-6758-4bf9-93b2-fbde7a4267f2.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189526525-7e69a486-a416-4fa9-a980-4e8fe9186bbc.svg" width="100px" />

# ColorID

[![Package version](https://img.shields.io/crates/v/colorid.svg)](https://crates.io/crates/colorid)
[![License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://github.com/rustq/colorid/blob/master/LICENSE)

`Color as Identify - The unique 4-colors-ID string generator in in Rust`

`一款基于 4 种颜色来表示身份的 Rust 随机 ID 生成器`

## Install

```toml
[dependencies]
colorid = "0.0.4"
```

## Usage

```rust
use colorid::colorid;

fn main() {
   let id = colorid!(); // #A3F68E-#33EFAF-#8CB823-#D82C91
}
```

`COLORID: #A3F68E-#33EFAF-#8CB823-#D82C91`

<img src="https://user-images.githubusercontent.com/11075892/189936058-c85b9f86-6cd8-4904-a8ae-1f802d696113.svg" width="300px">


## Benchmark

```rust
colorid                 time:   [735.43 ns 735.51 ns 735.61 ns]
uuid                    time:   [750.16 ns 750.33 ns 750.62 ns]
nanoid                  time:   [926.34 ns 926.45 ns 926.57 ns]
```

[run: 3045840932](https://github.com/rustq/colorid/actions/runs/3045840932/jobs/4907902169)

## License

[MIT](https://opensource.org/licenses/MIT)


## ColorID in Other Languages

- [JavaScript](https://github.com/rustq/colorid.js)
