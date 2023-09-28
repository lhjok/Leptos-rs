## 前端构建（Rust + Dioxus）
#### 1、安装第三方JS包
```sh
npm install
```
#### 2、在`.cargo`目录下创建`config.toml`
```toml
[build]
rustflags = ["--cfg=web_sys_unstable_apis"]
```
#### 3、在调试模式下运行
```sh
trunk serve
```
#### 4、在发布模式下构建
```sh
trunk build --release
```