<img src="https://user-images.githubusercontent.com/11075892/189526526-4a0a0049-1714-4ba4-a967-3a7f62d5e5e0.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189526521-ac0f8644-4915-4feb-b371-d9b3add3cb44.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189526523-4ae3ff55-6758-4bf9-93b2-fbde7a4267f2.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189526525-7e69a486-a416-4fa9-a980-4e8fe9186bbc.svg" width="100px" />

# ColorID

[![Package version](https://img.shields.io/crates/v/colorid.svg)](https://crates.io/crates/colorid)
[![License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://github.com/rustq/colorid/blob/master/LICENSE)

Color as Identify - The unique 4-colors-ID string generator in `Rust`

The performance of `ColorID` is better than `UUID` and `NanoID` (The algorithm of `ColorID` is actually very similar to the `UUID.V4`)

If we use `ColorID` to represent identities in social networks, we can easily generate personalized social information for users, such as avatars, NFTs, etc.

The `ColorID` is consisted by 4 colors in the RGB channel from 12 unsigned 8 bits numbers (`u8`), so the theoretical total of `ColorID` is `(2^8)^12` = `2^96` ≈ `7*10^28`, which means that even if the `ColorID` is generated for every 7.8 billion people in the world every second, it will take 300 billion years to use up all `ColorIDs`. And `ColorID` provides safety by using hardware random generator, can also be used in clusters.

Because the Four color theorem, if we need to color the regions of any avatars or NFTs so that no two adjacent regions have the same color, four colors are enough!

一款基于 4 种颜色来表示身份的 `Rust` 随机 ID 生成器。

在性能表现上 `ColorID` 比 `UUID`、`NanoID` 等库表现更好 (虽然 `ColorID` 的生成算法实际上和 `UUID.V4` 非常相似)。

如果我们用 `ColorID` 表示社交网络中的身份，可以轻松地为用户生成个性化的社交信息：如头像、数字藏品等。

实际上 `ColorID` 是由 4 种颜色在 RGB 通道内的 12 个 8 位无符号整型数字 (`u8`) 组成，故 `ColorID` 理论上的总数为 `(2^8)^12` = `2^96` ≈ `7*10^28`，也就是说即使每秒都为全球 78 亿人中的每个人都生成一个 `ColorID`，也要花 3000 亿年才会将所有 `ColorID` 用完。此外 `ColorID` 使用强随机函数来生成随机数，故也可在集群使用中提供安全性。

因为四色定理的原因，如果我们想要为头像或者数字藏品涂色且不会出现邻接区域颜色相同, 有四种颜色就足够了！

## Install

```toml
[dependencies]
colorid = "0.0.6"
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

- [WASM](https://github.com/rustq/colorid-wasm)
