## 前端构建（Rust + Leptos）
#### 1、设置编译环境
```toml
# 在 `.cargo` 目录下创建 `config.toml`
[build]
rustflags = ["--cfg=web_sys_unstable_apis"]
```
#### 2、在调试模式下运行
```sh
$ npm install
$ trunk serve
```
#### 3、在发布模式下构建
```sh
$ trunk build --release
```