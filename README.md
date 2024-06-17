# 用于Rust的Android API的高级封装

## 介绍

这个箱子提供对Android API的高级封装，依赖于jni-rs库，这包括以下内容：

1. 消除函数调用时的unsafe；
2. 本地化类型，例如把java原始类型boolean转换到rust的基本类型bool；
3. 规范命名规则，例如把以小写开头字母的驼峰命名式的函数统一为下划线命名法；
4. 添加必要的官方说明，以便于随时查阅；
5. 实现内存自动管理；
6. 简化调用，让代码更整洁。

当您如果在尝试封装类似的功能函数时，请优先考虑上述规则。

## 使用

1. 安装apk打包工具
   ```shell
   cargo install cargo-apk
   ```
2. 运行示例
   ```shell
   git clone https://gitcode.net/mzdk100/droid-wrap.git
   cd droid-wrap
   cargo apk run -p droid-wrap-example
   ```
3. 运行测试
   ```shell
   cargo apk run -p droid-wrap-test --all-features
   ```

如需了解更多信息，请查看example目录中的代码示例。

## 分类

这些功能函数的分类使用条件编译的方式被链接到程序中。使用时，请将这个箱子作为可选依赖项，并指定相关的feature，这是防止编译不必要的代码从而让程序体积的不断膨胀。

## 安卓(android)

### 安卓应用(android_app)

提供Activity类的常用API。

### 安卓内容(android_content)

android.content.Context的API等。

### 安卓视图(android_view)

android.view.View的API等。

### 安卓小部件(android_widget)

常用的一些UI小部件。

## java功能(java)

### java语言内置(java_lang)

实现了java.lang.String,java.lang.Integer等类型。

