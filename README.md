# ISMISM

基于rust的也许跨平台的快速打开主义主义视频的命令行程序

## 兼容性(x86_64)

|平台|是否兼容|
|:-:|:-:|
|Windows 10/11|是|
|Linux|是|
|MacOS|未测试|

## 使用

1. 在releases中下载压缩包
2. 解压，确保二进制文件与ismism.txt在同一目录下
3. 运行 `./ismism -h` 查看帮助
4. Mac用户或arm用户可以尝试下载源码编译

ismism [args]

Args:

- -a: 添加条目
- -m [ismism]: 修改条目
- -o [ismism]: 打开条目
- -f [regex]: 搜索条目,支持正则表达式

    ```shell
    $ ismism -f "4-2.*"   # 查询所有4-2开头的条目
    4-2             现实的正统化                                                            BV1aq4y1R7Lk
    4-2-1           合律组织                                                                BV1MU4y1g7cm
    ...
    ```

- -l: 列举条目
- -h: 帮助
