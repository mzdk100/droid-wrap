# 用于Rust的Android API的高级封装

[GitCode](https://gitcode.com/mzdk100/droid-wrap.git)
[GitHub](https://github.com/mzdk100/droid-wrap)
[crates.io](https://crates.io/crates/droid-wrap)
[docs.rs](https://docs.rs/droid-wrap/latest/droid_wrap/)

## 介绍

这个箱子提供对Android API的高级封装，依赖于jni-rs库，这包括以下内容：

1. 消除函数调用时的unsafe；
2. 本地化类型，例如把java原始类型boolean转换到rust的基本类型bool；
3. 规范命名规则，例如把以小写开头字母的驼峰命名式的函数统一为下划线命名法；
4. 添加必要的官方说明，以便于随时查阅；
5. 实现内存自动管理；
6. 简化调用，让代码更整洁。

当您如果在尝试封装类似的功能函数时，请优先考虑上述规则。

## 特点

- Java 动态代理，使用Rust函数实现java接口的方法；
- 提供常用的Android API；
- API调用符合rust代码规范；
- 拥有完整的测试，覆盖每个函数调用；
- 构建和使用简单；
- 多线程安全。

## 使用

1. 环境配置
   在使用之前需要确保配置Android SDK和Java环境。
    - ANDROID_HOME： 指向Android SDK的根目录路径。
    - ANDROID_BUILD_TOOLS_VERSION： 例如"35.0.0"。
    - ANDROID_API_LEVEL： 例如35。
    - JAVA_HOME： 指向jre或者jdk的根目录路径。
      注意：我们不使用javac来编译生成java字节码，所以无须使用完整的JDK环境，仅使用JRE（Java 运行时）即可。
2. 安装apk打包工具
   ```shell
   cargo install cargo-apk2
   ```
3. 运行示例
   ```shell
   git clone https://gitcode.net/mzdk100/droid-wrap.git
   cd droid-wrap
   cargo apk2 run -p droid-wrap-example --example activity-example
   cargo apk2 run -p droid-wrap-example --example java-example
   ```
4. 运行测试
   ```shell
   cargo apk2 run -p droid-wrap-test --all-features
   ```

如需了解更多信息，请查看example目录中的代码示例。

### 关于构建工具

[cargo-apk2](https://github.com/mzdk100/cargo-apk2)是一个更加轻量级的安卓apk打包工具，他从已经弃用的[cargo-apk](https://github.com/rust-mobile/cargo-apk)fork而来，cargo-apk2将持续维护，可放心使用。
同时本项目中提供了一个[cargo-aapt2](aapt2/README.md)的cargo扩展程序，如果您对打包apk有更高的需求，可以使用此扩展。


## 分类

这些功能函数的分类使用条件编译的方式被链接到程序中。使用时，请将这个箱子作为可选依赖项，并指定相关的feature，这是防止编译不必要的代码从而让程序体积的不断膨胀。

## 安卓(android)

### 安卓应用(android_app)

提供Activity类的常用API。

### 安卓内容(android_content)

1. android.content.Context;
2. android.content.ContextWrapper;
3. android.content.Intent。;
4. android.content.ComponentName;
5. android.content.ComponentName_WithComponentName;

### 安卓硬件(android_hardware)

1. android.hardware.Camera;
2. android.hardware.vibrator.Effect;
3. android.hardware.vibrator.EffectStrength;

### 安卓系统(android_os)

1. android.os.Bundle;
2. android.os.VibrationEffect;
3. android.os.Vibrator;
4. android.os.VibratorManager;
5. android.os.Build;
6. android.os.SystemProperties;
7. android.os.Build_VERSION_CODES;
8. android.os.Build_VERSION;
9. android.os.SystemProperties_Handle;

### 安卓内容提供者(android_provider)

1. android.provider.Settings;

### 安卓语音(android_speech)

android.speech.tts.TextToSpeech的API等。

### 安卓文本(android_text)

已经实现下列接口：

1. android.text.Editable;
2. android.text.InputType;
3. android.text.TextWatcher;

### 安卓视图(android_view)

android.view.View的API等（包括点击监听器的实现）。

1. android.view.View;
2. android.view.ViewGroup;
3. android.view.View_OnClickListener;
4. android.view.ViewGroup_LayoutParams;
5. android.view.ViewGroup_MarginLayoutParams;
6. android.view.ViewManager;
7. android.view.WindowManager;
8. android.view.Display;
9. android.view.inputmethod.InputMethodManager;
10. android.view.KeyEvent;
11. android.view.InputEvent;
12. android.view.inputmethod.EditorInfo;
13. android.view.View_OnLongClickListener;
14. android.view.ViewParent;
15. android.view.View_OnKeyListener;
16. android.view.Window;
17. android.view.ContextThemeWrapper;
18. android.view.SurfaceHolder;
19. android.view.Surface;

### 安卓小部件(android_widget)

常用的一些UI小部件：

1. android.widget.TextView;
2. android.widget.EditText;
3. android.widget.Button;
4. android.widget.LinearLayout;
5. android.widget.LinearLayout_LayoutParams;
6. android.widget.TextView_OnEditorActionListener;

## java功能(java)

### javaIO操作(java_io)

实现了java.io.File等。

1. java.io.File;
2. java.io.Serializable;

### java语言内置(java_lang)

实现了java.lang.String,java.lang.Integer等类型。

1. java.lang.Boolean;
2. java.lang.Float;
3. java.lang.Object;
4. java.lang.String;
5. java.lang.Integer;
6. java.lang.CharSequence;
7. java.lang.Runnable;
8. java.lang.System;
9. java.lang.reflect.Executable;
10. java.lang.Comparable;

### java缓冲区(java_nio)

实现了java.nio.ByteBuffer等类型。

1. java.nio.ByteBuffer;
2. java.nio.file.Path;