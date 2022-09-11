<img src="https://user-images.githubusercontent.com/11075892/189524235-e20dfc88-6986-468c-8920-349887e516ce.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189524254-ff66b01a-ea54-4025-8c99-fbc413814c8a.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189524263-537b3569-6c45-4ee7-8139-e0fddf9b2226.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189524266-aa3bb68c-0cbb-4120-916d-257ecc0db246.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189524271-e3c92f62-4ef1-4703-8438-640ae436c8fc.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189524273-4c569f99-3ba4-4c70-a22e-ed67b1be34f3.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189524274-d3b4faea-b1a1-42fd-a32b-a16ded1a007e.svg" width="100px" /><img src="https://user-images.githubusercontent.com/11075892/189524281-e6cf4b57-a932-4795-9dc7-22675d209af0.svg" width="100px" />

# Color ID

[![Package version](https://img.shields.io/crates/v/colorid.svg)](https://crates.io/crates/colorid)
[![License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://github.com/rustq/colorid/blob/master/LICENSE)

`The ColorID in Rust`

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
   let id = colorid!(); //=> D020B5-2BC144-5F308F-D2599A
}
```

## Color ID in Other Languages

- [JavaScript](https://github.com/rustq/colorid.js)