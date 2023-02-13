# 第一章

实现引导特权软件，支撑 `legacy::console_putchar`。

- 源码全部位于 [`src/main.rs`](src/main.rs)，但开始依赖外部*板块（crate）*，见 [`Cargo.toml`](Cargo.toml#L9)；
- 定制链接脚本在 [`build.rs`](build.rs) 中调用 [`linker`](/linker) 板块的相关定义以复用代码；
