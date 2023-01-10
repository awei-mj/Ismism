# ISMISM

基于rust的也许跨平台的快速打开主义主义视频的命令行程序

## 兼容性

|平台|是否兼容|
|:-:|:-:|
|Windows 10/11|是|
|Linux(Ubuntu 22.04)|是|
|MacOS|未测试|

## 使用

1. **请先在deserialize()与serialize()函数中修改文件路径！！！**
2. 前往[官网](https://www.rust-lang.org/zh-CN/tools/install)按照说明安装rust编译器(下个版本准备加入用户配置文件)
3. 在仓库根目录下运行 `cargo build --release`
4. 前往target/release/ismism(.exe)找到可执行文件
5. 运行 `ismism -h` 查看帮助

ismism [args]

Args:

- -a: 添加条目
- -m [ismism]: 修改条目
- -o [ismism]: 打开条目
- -f [regex]: 搜索条目,支持正则表达式
- -l: 列举条目
- -h: 帮助
