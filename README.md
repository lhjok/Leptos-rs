## 前端构建（Rust + Leptos）
```sh
# 在(.cargo)目录下创建(config.toml)并添加下面两行
# [build]
# rustflags = ["--cfg=web_sys_unstable_apis"]
# 安装第三方JS依赖
$ npm install
# 在调试模式下运行
$ trunk serve
# 在发布模式下构建（拷贝dist目录到后端）
$ trunk build --public-url static --release
# (static)目录是后端注册的静态资源路由，编译后默认生成的文件依然
# 保存在(dist)目录下，奇怪的是trunk并不会自动拷贝已生成的文件
# 到(static)目录，需要自己手动拷贝，但是(index.html)文件除外。
# /dist    --> 根目录（后端的根路由是索引文件）
# /dist/index.html    --> 根目录下的索引文件
# /dist/static    --> 根目录下静态资源目录
```