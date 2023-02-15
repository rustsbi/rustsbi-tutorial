# RustSBI 开发教程

[![CI](https://github.com/YdrMaster/rustsbi-tutorial/actions/workflows/workflow.yml/badge.svg?branch=main)](https://github.com/YdrMaster/rustsbi-tutorial/actions)
[![issue](https://img.shields.io/github/issues/YdrMaster/rustsbi-tutorial)](https://github.com/YdrMaster/rustsbi-tutorial/issues)
[![license](https://img.shields.io/github/license/YdrMaster/rustsbi-tutorial)](LICENSE)

本教程旨在全面介绍 SBI 开发技术并给出各个扩展在 Qemu/virt 上基于 [RustSBI](https://crates.io/crates/rustsbi) 的参考实现。

## 使用方法

- `cargo qemu --ch <n>`

  在 qemu 运行第 `n` 章的 SBI 软件。

- `cargo book --ch <n> [--open]`

  生成第 `n` 章的指导书，包含 `--open` 选项时，生成后将在浏览器打开。

## 进度

| 章节     | 代码 | 教程 | 主要内容
|:-------:|:---:|:---:|-
|[§1](ch1)|  ✓  |     | 简单的机器态裸机应用程序
|[§2](ch2)|  ✓  |     | 扩展裸机应用程序
|[§3](ch3)|     |     | [sbi-spec](https://crates.io/crates/sbi-spec)，内核的加载和引导以及 SBI §5(Legacy)
|[§4](ch4)|     |     | SBI §3(Binary) + §4(Base)
|[§5](ch5)|     |     | 使用 RustSBI + SBI §10(SRST)
|[§6](ch6)|     |     | SBI §6(TIME) + §7(sPI)
|[§7](ch7)|     |     | SBI §9(HSM)
|[§8](ch8)|     |     | SBI 多核支持
