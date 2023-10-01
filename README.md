## 前端构建（Rust + Leptos）
```sh
# 在(.cargo)目录下创建(config.toml)
# [build]
# rustflags = ["--cfg=web_sys_unstable_apis"]
# 安装第三方JS依赖
$ npm install
# 在调试模式下运行
$ trunk serve
# 在发布模式下构建
$ trunk build --release
```