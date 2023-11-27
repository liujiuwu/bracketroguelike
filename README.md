# 1.添加wasm32 target

```shell
rustup target add wasm32-unknown-unknown
```
# 2.安装 wasm-bindgen 工具
```shell
cargo install wasm-bindgen-cli
```

# 3.生成WebAssembly文件
```shell
./wasm/build.sh
```
如果是在windows下，请使用git-bash来执行，linux、macos 直接运行build.sh脚本即可。
此脚本会在target/wasm目录下生成WebAssembly文件。


# 4.安装 miniserver 工具
```shell
cargo install miniserve
```

# 5.启动Web服务
```shell
#服务启动成功后，根据地址在浏览器访问即可。
miniserve target/wasm --index index.html
```