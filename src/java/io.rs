/*
 * Copyright (c) 2024. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */

use crate::{
    java::{lang::Comparable, nio::file::Path},
    JObjNew, JObjRef, JType,
};
use droid_wrap_derive::{java_class, java_constructor, java_field, java_method};

/**
文件和目录路径名的抽象表示。用户界面和操作系统使用系统相关的路径名字符串来命名文件和目录。
此类提供抽象的、独立于系统的分层路径名视图。抽象路径名有两个组成部分：
可选的系统相关前缀字符串，例如磁盘驱动器说明符，UNIX 根目录的“/”或 Microsoft Windows UNC 路径名的“\\\\”，以及零个或多个字符串名称的序列。
抽象路径名中的第一个名称可能是目录名，或者在 Microsoft Windows UNC 路径名的情况下是主机名。抽象路径名中的每个后续名称表示目录；最后一个名称可以表示目录或文件。
空的抽象路径名没有前缀和空的名称序列。路径名字符串与抽象路径名之间的转换本质上是系统相关的。
当抽象路径名转换为路径名字符串时，每个名称与下一个名称之间都由默认分隔符的单个副本分隔。默认的名称分隔符由系统属性 file.separator 定义，并可在此类的公共静态字段 Separator 和 SeparatorChar 中使用。
将路径名字符串转换为抽象路径名时，其中的名称可以用默认的名称分隔符或底层系统支持的任何其他名称分隔符分隔。路径名（无论是抽象的还是字符串形式的）可以是绝对的，也可以是相对的。
绝对路径名是完整的，因为不需要其他信息即可找到它所表示的文件。相反，相对路径名必须根据从其他路径名中获取的信息进行解释。
默认情况下，java.io 包中的类始终根据当前用户目录解析相对路径名。此目录由系统属性 user.dir 命名，通常是调用 Java 虚拟机的目录。
可以通过调用此类的 getParent 方法获取抽象路径名的父级，该父级由路径名的前缀和路径名名称序列中除最后一个之外的每个名称组成。
每个目录的绝对路径名都是任何具有绝对抽象路径名的 File 对象的祖先，该绝对抽象路径名以目录的绝对路径名开头。例如，抽象路径名“/usr”表示的目录是路径名“/usr/local/bin”表示的目录的祖先。
前缀概念用于处理 UNIX 平台上的根目录以及 Microsoft Windows 平台上的驱动器说明符、根目录和 UNC 路径名，如下所示：
对于 UNIX 平台，绝对路径名的前缀始终为“/”。相对路径名没有前缀。表示根目录的抽象路径名具有前缀“/”和空名称序列。
对于 Microsoft Windows 平台，包含驱动器说明符的路径名的前缀由驱动器号后跟“:”组成，如果路径名是绝对路径名，则可能后跟“\\”。
UNC 路径名的前缀为“\\\\”；主机名和共享名是名称序列中的前两个名称。未指定驱动器的相对路径名没有前缀。
此类的实例可能表示也可能不表示实际的文件系统对象，例如文件或目录。如果它表示这样的对象，则该对象驻留在分区中。
分区是文件系统的操作系统特定存储部分。单个存储设备（例如物理磁盘驱动器、闪存、CD-ROM）可能包含多个分区。
对象（如果有）将驻留在此路径名的绝对形式的某个祖先命名的分区上。文件系统可以对实际文件系统对象的某些操作实施限制，例如读取、写入和执行。这些限制统称为访问权限。
文件系统可能对单个对象具有多组访问权限。例如，一组可能适用于对象的所有者，而另一组可能适用于所有其他用户。对象上的访问权限可能会导致此类中的某些方法失败。
File 类的实例是不可变的；也就是说，一旦创建，File 对象所表示的抽象路径名就永远不会改变。

与 java.nio.file 包的互操作性
java.nio.file 包定义了 Java 虚拟机访问文件、文件属性和文件系统的接口和类。此 API 可用于克服 java.io.File 类的许多限制。
toPath 方法可用于获取使用 File 对象所表示的抽象路径来定位文件的 Path。生成的 Path 可与 java.nio.file.Files 类一起使用，以提供对其他文件操作、文件属性和 I/O 异常的更高效和广泛的访问，以帮助诊断对文件的操作时出现的错误
*/
#[java_class(name = "java/io/File", extends = super::lang::Object)]
pub struct File;

impl File {
    /// 当应用以 SDK 级别 35（Android 15）或更高级别为目标时，规范化根目录的父目录。根据 POSIX，“/..”可以规范化为“/”。
    pub const CANONICALIZE_PARENT_OF_ROOT_DIR: u64 = 312399441;

    /**
    通过将给定的路径名​​字符串转换为抽象路径名来创建新的 File 实例。如果给定的字符串是空字符串，则结果为空的抽象路径名。
    抛出 NullPointerException – 如果 pathname 参数为 null
    `pathname` 路径名字符串
    */
    #[java_constructor]
    pub fn new(pathname: String) -> Result<Self, <Self as JType>::Error> {}

    /**
    根据父路径名字符串和子路径名字符串创建一个新的 File 实例。
    如果父路径名字符串为 null，则新的 File 实例的创建方式与对给定的子路径名字符串调用单参数 File 构造函数的方式相同。否则，父路径名字符串将被视为目录，而子路径名字符串将被视为目录或文件。
    如果子路径名字符串是绝对路径，则将以系统相关的方式将其转换为相对路径名。如果父路径名字符串为空字符串，则通过将子路径名转换为抽象路径名并根据系统相关的默认目录解析结果来创建新的 File 实例。
    否则，每个路径名字符串都将转换为抽象路径名，并根据父路径名解析子抽象路径名。
    抛出：NullPointerException - 如果子路径名字符串为 null
    `parent` 父路径名字符串
    `child` 子路径名字符串
    */
    #[java_constructor]
    pub fn new_with_parent(
        parent: Option<String>,
        child: String,
    ) -> Result<Self, <Self as JType>::Error> {
    }

    /**
    根据父抽象路径名和子路径名字符串创建新的 File 实例。如果父路径为 null，则创建新的 File 实例，就像通过对给定的子路径名字符串调用单参数 File 构造函数一样。
    否则，父抽象路径名将被视为目录，而子路径名字符串将被视为目录或文件。如果子路径名字符串是绝对路径，则以系统相关的方式将其转换为相对路径名。
    如果父路径为空抽象路径名，则通过将子路径转换为抽象路径名并根据系统相关的默认目录解析结果来创建新的 File 实例。否则，每个路径名字符串都将转换为抽象路径名，并根据父路径解析子抽象路径名。
    抛出：NullPointerException – 如果子路径为 null
    `parent` 父抽象路径名
    `child` 子路径名字符串
    */
    #[java_constructor]
    pub fn new_with_parent_file(
        parent: Option<Self>,
        child: String,
    ) -> Result<Self, <Self as JType>::Error> {
    }

    /**
    系统相关的默认名称分隔符。此字段初始化为包含系统属性文件值的第一个字符。分隔符。
    在 UNIX 系统上，此字段的值为 '/'；在 Microsoft Windows 系统上，此字段的值为 '\\'。
    */
    #[java_field]
    pub fn get_separator_char() -> char {}

    /**
    系统相关的默认名称分隔符，为方便起见，以字符串表示。
    此字符串包含单个字符，即分隔符。
    */
    #[java_field]
    pub fn get_separator() -> String {}

    /**
    系统相关的路径分隔符。此字段初始化为包含系统属性路径值的第一个字符。
    分隔符。此字符用于分隔以路径列表形式给出的文件序列中的文件名。在 UNIX 系统上，此字符为 ':'；在 Microsoft Windows 系统上，此字符为 ';'。
    */
    #[java_field]
    pub fn get_path_separator_char() -> char {}

    /**
    系统相关的路径分隔符，为方便起见，以字符串表示。
    此字符串包含单个字符，即 pathSeparatorChar。
    */
    #[java_field]
    pub fn get_path_separator() -> String {}

    /**
    返回此抽象路径名表示的文件或目录的名称。这只是路径名的名称序列中的最后一个名称。
    如果路径名的名称序列为空，则返回空字符串。
    返回：此抽象路径名表示的文件或目录的名称，如果此路径名的名称序列为空，则返回空字符串
    */
    #[java_method]
    pub fn get_name(&self) -> String {}

    /**
    返回此抽象路径名的父级路径名字符串，如果此路径名未指定父目录，则返回 null。
    抽象路径名的父级由路径名的前缀（如果有）和路径名名称序列中除最后一个之外的每个名称组成。如果名称序列为空，则路径名未指定父目录。
    返回：此抽象路径名指定的父目录的路径名字符串，如果此路径名未指定父目录，则返回 null
    */
    #[java_method]
    pub fn get_parent(&self) -> Option<String> {}

    /**
    返回此抽象路径名的父级的抽象路径名，如果此路径名未指定父目录，则返回 null。
    抽象路径名的父级由路径名的前缀（如果有）和路径名名称序列中除最后一个之外的每个名称组成。如果名称序列为空，则路径名未指定父目录。
    返回：此抽象路径名指定的父目录的抽象路径名，如果此路径名未指定父目录，则返回 null
    */
    #[java_method]
    pub fn get_parent_file(&self) -> Option<Self> {}

    /**
    测试此抽象路径名是否为绝对路径名。绝对路径名的定义取决于系统。
    在 Android 上，绝对路径以字符“/”开头。
    返回：如果此抽象路径名是绝对路径名，则返回 true，否则返回 false
    */
    #[java_method]
    pub fn is_absolute(&self) -> bool {}

    /**
    返回此文件的绝对路径。绝对路径是从文件系统根目录开始的路径。
    在 Android 上，只有一个根目录：/。
    绝对路径的常见用途是将路径作为命令行参数传递给进程，以消除相对路径所暗示的要求，即子进程必须具有与其父进程相同的工作目录。
    返回：表示与此抽象路径名相同的文件或目录的绝对路径名字符串
    */
    #[java_method]
    pub fn get_absolute_path(&self) -> String {}

    /**
    返回此抽象路径名的规范路径名字符串。规范路径名既是绝对的又是唯一的。
    规范形式的精确定义取决于系统。此方法首先将此路径名转换为绝对形式（如通过调用 getAbsolutePath 方法），然后以系统相关的方式将其映射到其唯一形式。
    这通常涉及从路径名中删除冗余名称（例如“。”和“..”）、解析符号链接（在 UNIX 平台上）以及将驱动器号转换为标准大小写（在 Microsoft Windows 平台上）。
    表示现有文件或目录的每个路径名都具有唯一的规范形式。表示不存在的文件或目录的每个路径名也具有唯一的规范形式。
    不存在的文件或目录的路径名的规范形式可能与创建文件或目录后相同路径名的规范形式不同。类似地，现有文件或目录的路径名的规范形式可能与文件或目录被删除后相同路径名的规范形式不同。
    返回：表示与此抽象路径名相同的文件或目录的规范路径名字符串
    抛出：
    - IOException – 如果发生 I/O 错误，这是可能的，因为规范路径名的构造可能需要文件系统查询
    - SecurityException – 如果无法访问所需的系统属性值，或者存在安全管理器并且其 SecurityManager. checkRead 方法拒绝对文件的读取访问
    */
    #[java_method]
    pub fn get_canonical_path(&self) -> Result<String, <Self as JType>::Error> {}

    /**
    返回此抽象路径名的规范形式。等效于新文件（this.getCanonicalPath）。
    返回：与此抽象路径名相同的文件或目录表示相同的文件或目录的规范路径名字符串
    抛出：
    - IOException - 如果发生I/ O错误，这是可能的，因为规范路径名的构建可能需要文件系统查询
    - SecurityException - 如果无法访问所需的系统属性值，或者存在安全管理器及其SecurityManager。检查方法拒绝阅读对文件的访问
    */
    #[java_method]
    pub fn get_canonical_file(&self) -> Result<Self, <Self as JType>::Error> {}

    /**
    测试应用程序是否可以读取此抽象路径名表示的文件。
    返回：当且仅当此抽象路径名指定的文件存在且可由应用程序读取时，才返回 true；否则返回 false
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager. checkRead(String) 方法拒绝对文件的读取访问
    */
    #[java_method]
    pub fn can_read(&self) -> Result<bool, <Self as JType>::Error> {}

    /**
    测试应用程序是否可以修改此抽象路径名表示的文件。
    返回：当且仅当文件系统实际包含此抽象路径名表示的文件且允许应用程序写入该文件时，才返回 true；否则返回 false。
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager. checkWrite(String) 方法拒绝对文件的写访问
    */
    #[java_method]
    pub fn can_write(&self) -> Result<bool, <Self as JType>::Error> {}

    /**
    测试此抽象路径名表示的文件或目录是否存在。
    返回：当且仅当此抽象路径名表示的文件或目录存在时，返回 true；否则返回 false
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager. checkRead(String) 方法拒绝对文件或目录进行读取访问
    */
    #[java_method]
    pub fn exists(&self) -> Result<bool, <Self as JType>::Error> {}

    /**
    测试此抽象路径名表示的文件是否为目录。如果需要区分 I/O 异常和文件不是目录的情况，或者需要同时获取同一文件的多个属性，则可以使用 Files.readAttributes 方法。
    返回：当且仅当此抽象路径名表示的文件存在且为目录时，才返回 true；否则返回 false
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager. checkRead(String) 方法拒绝对文件的读取访问
    */
    #[java_method]
    pub fn is_directory(&self) -> Result<bool, <Self as JType>::Error> {}

    /**
    测试此抽象路径名表示的文件是否为普通文件。如果文件不是目录，并且还满足其他系统相关标准，则该文件为普通文件。
    Java 应用程序创建的任何非目录文件都保证是普通文件。如果需要区分 I/O 异常和文件不是普通文件的情况，或者需要同时使用同一文件的多个属性，则可以使用 Files.readAttributes 方法。
    返回：当且仅当此抽象路径名表示的文件存在且为普通文件时，才返回 true；否则返回 false
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager.checkRead(String) 方法拒绝对文件的读取访问
    */
    #[java_method]
    pub fn is_file(&self) -> Result<bool, <Self as JType>::Error> {}

    /**
    测试此抽象路径名所命名的文件是否为隐藏文件。隐藏的确切定义取决于系统。
    在 UNIX 系统上，如果文件名称以句点字符 ('.') 开头，则认为文件是隐藏的。
    在 Microsoft Windows 系统上，如果文件在文件系统中被标记为隐藏，则认为文件是隐藏的。
    当且仅当此抽象路径名所表示的文件根据底层平台的约定是隐藏的，则返回 true
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager. checkRead(String) 方法拒绝对文件的读取访问
    */
    #[java_method]
    pub fn is_hidden(&self) -> Result<bool, <Self as JType>::Error> {}

    /**
    返回此抽象路径名表示的文件的最后修改时间。如果需要区分 I/O 异常和返回 0L 的情况，或者需要同时获取同一文件的多个属性，或者需要上次访问时间或创建时间，则可以使用 Files.readAttributes 方法。
    返回：表示文件最后修改时间的长整型值，以自纪元 (1970 年 1 月 1 日 00:00:00 GMT) 以来的毫秒数计算，如果文件不存在或发生 I/O 错误，则返回 0L
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager. checkRead(String) 方法拒绝对文件的读取访问
    */
    #[java_method]
    pub fn last_modified(&self) -> Result<u64, <Self as JType>::Error> {}

    /**
    返回此抽象路径名表示的文件的长度。如果此路径名表示目录，则返回值未指定。
    如果需要区分 I/O 异常和返回 0L 的情况，或者需要同时获取同一文件的多个属性，则可以使用 Files.readAttributes 方法。
    返回：此抽象路径名表示的文件的长度（以字节为单位），如果文件不存在，则返回 0L。某些操作系统可能会为表示系统相关实体（如设备或管道）的路径名返回 0L。
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager. checkRead(String) 方法拒绝对文件的读取访问
    */
    #[java_method]
    pub fn length(&self) -> Result<u64, <Self as JType>::Error> {}

    /**
    当且仅当具有此名称的文件尚不存在时，原子地创建一个以此抽象路径名命名的新空文件。
    检查文件是否存在以及如果文件不存在则创建文件是一个单一操作，该操作对于可能影响文件的所有其他文件系统活动而言都是原子的。
    注意：此方法不应用于文件锁定，因为生成的协议无法可靠地工作。应改用 FileLock 工具。
    返回：如果命名文件不存在且已成功创建，则返回 true；如果命名文件已存在，则返回 false
    抛出：
    - IOException – 如果发生 I/O 错误
    - SecurityException – 如果安全管理器存在并且其 SecurityManager. checkWrite(String) 方法拒绝对文件的写访问
    */
    #[java_method]
    pub fn create_new_file(&self) -> Result<bool, <Self as JType>::Error> {}

    /**
    删除此抽象路径名表示的文件或目录。如果此路径名表示目录，则目录必须为空才能删除。
    请注意，java.nio.file.Files 类定义 delete 方法，当无法删除文件时抛出 IOException。这对于错误报告和诊断无法删除文件的原因很有用。
    返回：当且仅当成功删除文件或目录时才返回 true；否则返回 false
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager.checkDelete 方法拒绝删除文件
    */
    #[java_method]
    pub fn delete(&self) -> Result<bool, <Self as JType>::Error> {}

    /**
    请求在虚拟机终止时删除由此抽象路径名表示的文件或目录。文件（或目录）的删除顺序与注册顺序相反。
    调用此方法删除已注册为删除的文件或目录无效。只有在虚拟机正常终止时才会尝试删除，如 Java 语言规范所定义。
    一旦请求删除，就无法取消请求。因此，应谨慎使用此方法。
    注意：不应将此方法用于文件锁定，因为生成的协议无法可靠地工作。应改用 FileLock 工具。
    请注意，在 Android 上，应用程序生命周期不包括 VM 终止，因此调用此方法不能确保文件被删除。相反，您应该使用以下最合适的方法：
    - 使用 finally 子句手动调用删除。
    - 维护要删除的您自己的文件集，并在应用程序生命周期的适当时间点对其进行处理。
    - 使用 Unix 技巧，在所有读取器和写入器打开文件后立即删除文件。
    没有新的读者/写者将能够访问该文件，但所有现有的读者/写者仍将拥有访问权限，直到最后一个读者/写者关闭该文件。
    抛出:SecurityException – 如果存在安全管理器，并且其 SecurityManager. checkDelete 方法拒绝删除该文件的权限
    */
    #[java_method]
    pub fn delete_on_exit(&self) -> Result<(), <Self as JType>::Error> {}

    /**
    创建以此抽象路径名命名的目录。
    返回：当且仅当目录已创建时才返回 true；否则返回 false
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager. checkWrite(String) 方法不允许创建命名的目录
    */
    #[java_method]
    pub fn mkdir(&self) -> Result<bool, <Self as JType>::Error> {}

    //noinspection SpellCheckingInspection
    /**
    创建由此抽象路径名命名的目录，包括任何必要但不存在的父目录。
    请注意，如果此操作失败，它可能已成功创建一些必要的父目录。
    返回：当且仅当创建了目录以及所有必要的父目录时，才返回 true；否则返回 false
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager.checkRead(String) 方法不允许验证指定目录和所有必要的父目录的存在；或者如果 SecurityManager.checkWrite(String) 方法不允许创建指定目录和所有必要的父目录
    */
    #[java_method]
    pub fn mkdirs(&self) -> Result<bool, <Self as JType>::Error> {}

    /**
    重命名此抽象路径名表示的文件。可能会出现许多失败情况。一些更可能发生的失败情况包括：
    - 需要对包含源路径和目标路径的目录具有写入权限。
    - 需要对两个路径的所有父级具有搜索权限。
    - 两个路径都位于同一挂载点。在 Android 上，应用程序在尝试在内部存储和 SD 卡之间复制时最有可能遇到此限制。
    应始终检查返回值以确保重命名操作成功。请注意，java.nio.file.Files 类定义了 move 方法，以独立于平台的方式移动或重命名文件。
    返回：当且仅当重命名成功时返回 true；否则返回 false
    抛出：
    - SecurityException – 如果存在安全管理器，并且其 SecurityManager.checkWrite(String) 方法拒绝对旧路径名或新路径名进行写访问
    - NullPointerException – 如果参数 dest 为 null
    `dest` 命名文件的新抽象路径名
    */
    #[java_method]
    pub fn rename_to(&self, dest: Self) -> Result<bool, <Self as JType>::Error> {}

    /**
    设置由此抽象路径名命名的文件或目录的最后修改时间。所有平台都支持将文件修改时间精确到秒，但有些平台提供更高的精度。
    参数将被截断以适应支持的精度。如果操作成功并且文件上没有中间操作，则下次调用 lastModified 方法将返回传递给此方法的（可能被截断的）时间参数。
    `time` 新的最后修改时间，以自纪元（00:00:00 GMT，1970 年 1 月 1 日）以来的毫秒数为单位
    返回：当且仅当操作成功时返回 true；否则返回 false
    抛出：
    - IllegalArgumentException - 如果参数为负数
    - SecurityException - 如果存在安全管理器并且其 SecurityManager. checkWrite(String) 方法拒绝对指定文件的写访问
    */
    #[java_method]
    pub fn set_last_modified(&self, time: u64) -> Result<bool, <Self as JType>::Error> {}

    /**
    标记由此抽象路径名命名的文件或目录，以便只允许读取操作。
    调用此方法后，文件或目录将不会更改，直到将其删除或标记为允许写入访问。是否可以删除只读文件或目录取决于底层系统。
    返回：当且仅当操作成功时才​​返回 true；否则返回 false
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager. checkWrite(String) 方法拒绝对指定文件的写入访问
    */
    #[java_method]
    pub fn set_read_only(&self) -> Result<bool, <Self as JType>::Error> {}

    /**
    设置此抽象路径名的所有者或所有人的写权限。
    java.nio.file.Files 类定义操作文件属性（包括文件权限）的方法。当需要更精细地操作文件权限时，可以使用该方法。
    返回：当且仅当操作成功时才​​为 true。如果用户无权更改此抽象路径名的访问权限，则操作将失败。
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager. checkWrite(String) 方法拒绝对指定文件的写访问
    `writable` 如果为 true，则设置访问权限以允许写操作；如果为 false，则不允许写操作
    `owner_only` 如果为 true，则写权限仅适用于所有者的写权限；否则，它适用于所有人。如果底层文件系统无法区分所有者的写权限与其他人的写权限，则无论此值如何，该权限都将适用于所有人。
    */
    #[java_method]
    pub fn set_writable(
        &self,
        writable: bool,
        owner_only: bool,
    ) -> Result<bool, <Self as JType>::Error> {
    }

    /**
    一种便捷方法，用于设置此抽象路径名的所有者的写入权限。调用 file.setWritable(arg) 形式的此方法的行为与调用 file.setWritable(arg, true) 完全相同
    返回：true，当且仅当操作成功时。如果用户无权更改此抽象路径名的访问权限，则操作将失败。
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager.checkWrite(String) 方法拒绝对文件的写入访问
    `writable` 如果为 true，则设置访问权限以允许写入操作；如果为 false，则不允许写入操作
    */
    #[java_method(overload = setWritable)]
    pub fn set_writable_convenience(&self, writable: bool) -> Result<bool, <Self as JType>::Error> {
    }

    /**
    设置此抽象路径名的所有者或所有人的读取权限。 java.nio.file.Files 类定义操作文件属性（包括文件权限）的方法。
    当需要更精细地操作文件权限时，可以使用该方法。
    当且仅当操作成功时，返回 true。如果用户无权更改此抽象路径名的访问权限，则操作将失败。如果 readable 为 false，并且底层文件系统未实现读取权限，则操作将失败。
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager. checkWrite(String) 方法拒绝对文件的写访问
    `readable` 如果为 true，则设置访问权限以允许读取操作；如果为 false，则不允许读取操作
    `owner_only` 如果为 true，则读取权限仅适用于所有者的读取权限；否则，适用于所有人。如果底层文件系统无法区分所有者的读取权限与其他人的读取权限，则无论此值如何，该权限都将适用于所有人。
    */
    #[java_method]
    pub fn set_readable(
        &self,
        readable: bool,
        owner_only: bool,
    ) -> Result<bool, <Self as JType>::Error> {
    }

    /**
    一种便捷方法，用于设置此抽象路径名的所有者的读取权限。调用 file.setReadable(arg) 形式的此方法的行为与调用 file.setReadable(arg, true) 完全相同
    当且仅当操作成功时，返回 true。如果用户无权更改此抽象路径名的访问权限，则操作将失败。如果 readable 为 false，并且底层文件系统未实现读取权限，则操作将失败。
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager.checkWrite(String) 方法拒绝对文件的写访问
    `readable` 如果为 true，则设置访问权限以允许读取操作；如果为 false，则不允许读取操作
    */
    #[java_method(overload = setReadable)]
    pub fn set_readable_convenience(&self, readable: bool) -> Result<bool, <Self as JType>::Error> {
    }

    /**
    设置此抽象路径名的所有者或所有人的执行权限。 java.nio.file.Files 类定义对文件属性（包括文件权限）进行操作的方法。
    当需要更精细地操作文件权限时，可以使用此方法。
    返回：true 当且仅当操作成功时。如果用户无权更改此抽象路径名的访问权限，则操作将失败。如果 executable 为 false，并且底层文件系统未实现执行权限，则操作将失败。
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager. checkWrite(String) 方法拒绝对文件的写访问
    `executable` 如果为 true，则设置访问权限以允许执行操作；如果为 false，则不允许执行操作
    `owner_only` 如果为 true，则执行权限仅适用于所有者的执行权限；否则，它适用于所有人。如果底层文件系统无法区分所有者的执行权限与其他人的执行权限，则无论此值如何，该权限都将适用于所有人。
    */
    #[java_method]
    pub fn set_executable(
        &self,
        executable: bool,
        owner_only: bool,
    ) -> Result<bool, <Self as JType>::Error> {
    }

    /**
    一种便捷方法，用于设置此抽象路径名的所有者的执行权限。调用 file.setExecutable(arg) 形式的此方法的行为与调用 file.setExecutable(arg, true) 完全相同
    返回：true 当且仅当操作成功时。如果用户无权更改此抽象路径名的访问权限，则操作将失败。如果 executable 为 false 并且底层文件系统未实现执行权限，则操作将失败。
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager.checkWrite(String) 方法拒绝对文件的写访问
    `executable` 如果为 true，则设置访问权限以允许执行操作；如果为 false，则不允许执行操作
    */
    #[java_method(overload = setExecutable)]
    pub fn set_executable_convenience(
        &self,
        executable: bool,
    ) -> Result<bool, <Self as JType>::Error> {
    }

    /**
    测试应用程序是否可以执行此抽象路径名表示的文件。
    返回：当且仅当抽象路径名存在且应用程序被允许执行文件时，才返回 true
    抛出：SecurityException – 如果存在安全管理器，并且其 SecurityManager. checkExec(String) 方法拒绝对文件的执行访问
    */
    #[java_method]
    pub fn can_execute(&self) -> Result<bool, <Self as JType>::Error> {}

    /**
    返回此抽象路径名命名的分区的大小。
    返回：分区的大小（以字节为单位），如果此抽象路径名未命名分区，则返回 0L
    抛出：SecurityException – 如果已安装安全管理器并且它拒绝 RuntimePermission("getFileSystemAttributes") 或其 SecurityManager。checkRead(String) 方法拒绝对此抽象路径名命名的文件进行读取访问
    */
    #[java_method]
    pub fn get_total_space(&self) -> Result<u64, <Self as JType>::Error> {}

    /**
    返回此抽象路径名所命名的分区中未分配的字节数。返回的未分配字节数是一个提示，但不能保证可以使用其中大部分或任何字节。
    未分配字节数很可能在此调用后立即准确。任何外部 I/O 操作（包括在此虚拟机之外的系统上执行的操作）都可能使它变得不准确。
    此方法不保证对此文件系统的写入操作会成功。
    返回：分区上未分配的字节数，如果抽象路径名未命名分区，则返回 0L。此值将小于或等于 getTotalSpace 返回的总文件系统大小。
    抛出：SecurityException – 如果已安装安全管理器并且它拒绝 RuntimePermission("getFileSystemAttributes") 或其 SecurityManager。checkRead(String) 方法拒绝对此抽象路径名所命名的文件进行读取访问
    */
    #[java_method]
    pub fn get_free_space(&self) -> Result<u64, <Self as JType>::Error> {}

    /**
    返回此虚拟机在由该抽象路径名命名的分区上可用的字节数。如果可能，此方法会检查写入权限和其他操作系统限制，因此通常会比 getFreeSpace 更准确地估计实际可以写入多少新数据。
    返回的可用字节数是一个提示，但不能保证可以使用其中大部分或任何字节。未分配字节数很可能在此调用后立即准确。任何外部 I/O 操作（包括在此虚拟机之外的系统上执行的操作）都可能使它不准确。
    此方法不保证对此文件系统的写入操作会成功。在 Android（和其他基于 Unix 的系统）上，此方法返回非 root 用户可用的空闲字节数，无论您是否实际以 root 身份运行，也不管可能对用户适用任何配额或其他限制。 （getFreeSpace 方法返回根目录可能可用的字节数。）
    返回：分区上可用的字节数，如果抽象路径名未指定分区，则返回 0L。在没有此信息的系统中，此方法相当于调用 getFreeSpace。
    抛出：SecurityException – 如果已安装安全管理器，并且它拒绝 RuntimePermission("getFileSystemAttributes") 或其 SecurityManager。checkRead(String) 方法拒绝对此抽象路径名命名的文件进行读取访问
    */
    #[java_method]
    pub fn get_usable_space(&self) -> Result<u64, <Self as JType>::Error> {}

    /**
    在指定目录中创建一个新的空文件，使用给定的前缀和后缀字符串生成其名称。如果此方法成功返回，则保证：在调用此方法之前，返回的抽象路径名表示的文件不存在，并且在虚拟机的当前调用中，此方法及其任何变体都不会再次返回相同的抽象路径名。
    此方法仅提供临时文件功能的一部分。要安排自动删除此方法创建的文件，请使用 deleteOnExit 方法。
    前缀参数必须至少为三个字符长。建议前缀为简短、有意义的字符串，例如“hjb”或“mail”。后缀参数可以为空，在这种情况下将使用后缀“.tmp”。
    要创建新文件，可以先调整前缀和后缀以适应底层平台的限制。如果前缀太长，则会被截断，但其前三个字符将始终保留。如果后缀太长，它也会被截断，但如果它以句点字符 ('.') 开头，那么句点和其后的前三个字符将始终保留。
    完成这些调整后，将通过连接前缀、五个或更多内部生成的字符和后缀来生成新文件的名称。如果目录参数为 null，则将使用系统相关的默认临时文件目录。
    默认临时文件目录由系统属性 java.io.tmpdir 指定。在 UNIX 系统上，此属性的默认值通常为“/tmp”或“/var/tmp”；在 Microsoft Windows 系统上，它通常为“C:\\WINNT\\TEMP”。
    调用 Java 虚拟机时，可以为此系统属性赋予不同的值，但对此属性的编程更改不能保证对此方法使用的临时目录产生任何影响。
    返回：表示新创建的空文件的抽象路径名
    抛出：
    - IllegalArgumentException – 如果前缀参数包含的字符少于三个
    - IOException – 如果无法创建文件
    - SecurityException – 如果存在安全管理器并且其 SecurityManager. checkWrite(String) 方法不允许创建文件
    `prefix` 用于生成文件名的前缀字符串；必须至少为三个字符长
    `suffix` 用于生成文件名的后缀字符串；可以为 null，在这种情况下将使用后缀“.tmp”
    `directory` 要在其中创建文件的目录，如果要使用默认临时文件目录，则为 null
    */
    #[java_method]
    pub fn create_temp_file(
        prefix: String,
        suffix: Option<String>,
        directory: Option<Self>,
    ) -> Result<Self, <Self as JType>::Error> {
    }

    /**
    在默认临时文件目录中创建一个空文件，使用给定的前缀和后缀生成其名称。调用此方法相当于调用 createTempFile(prefix, suffix, null)。
    Files.createTempFile 方法提供了一种在临时文件目录中创建空文件的替代方法。该方法创建的文件可能对该方法创建的文件具有更严格的访问权限，因此可能更适合安全敏感的应用程序。
    返回：表示新创建的空文件的抽象路径名
    抛出：
    - IllegalArgumentException – 如果前缀参数包含的字符少于三个
    - IOException – 如果无法创建文件
    - SecurityException – 如果存在安全管理器及其 SecurityManager。 checkWrite(String) 方法不允许创建文件
    `prefix` 用于生成文件名称的前缀字符串；必须至少有三个字符长
    `suffix` 用于生成文件名称的后缀字符串；可以为 null，在这种情况下将使用后缀“.tmp”
     */
    #[java_method(overload = createTempFile)]
    pub fn create_temp_file_default(
        prefix: String,
        suffix: Option<String>,
    ) -> Result<Self, <Self as JType>::Error> {
    }

    /**
    返回一个从此抽象路径构造的 java.nio.file.Path 对象。生成的 Path 与默认文件系统相关联。第一次调用此方法就像调用它相当于评估表达式：
    FileSystems.getDefault().getPath(this.getPath());
    后续调用此方法将返回相同的 Path。如果此抽象路径名是空的抽象路径名，则此方法将返回可用于访问当前用户目录的 Path。
    返回：从此抽象路径构造的 Path
    抛出：InvalidPathException – 如果无法从抽象路径构造 Path 对象（请参阅 FileSystem. getPath）
    */
    #[java_method]
    pub fn to_path<P: Path>(&self) -> Result<P, <Self as JType>::Error> {}
}

impl Comparable<File> for File {
    /**
    按字典顺序比较两个抽象路径名。此方法定义的顺序取决于底层系统。
    在 UNIX 系统上，字母大小写在比较路径名时很重要；在 Microsoft Windows 系统上则不然。
    返回：如果参数等于此抽象路径名，则返回零；如果此抽象路径名按字典顺序小于参数，则返回小于零的值；如果此抽象路径名按字典顺序大于参数，则返回大于零的值
    `pathname` 要与此抽象路径名进行比较的抽象路径名
    */
    #[java_method]
    fn compare_to(&self, o: &File) -> Result<i32, <Self as JType>::Error> {}
}

/// 测试java.io
#[cfg(feature = "test_java_io")]
pub fn test() {
    assert_eq!('/', File::get_separator_char());
    assert_eq!(String::from("/"), File::get_separator());
    assert_eq!(':', File::get_path_separator_char());
    assert_eq!(String::from(":"), File::get_path_separator());
    let file = File::new("/data".to_string()).unwrap();
    let file = File::new_with_parent(Some(file.to_string()), "/local".to_string()).unwrap();
    let file = File::new_with_parent_file(Some(file), "/tmp".to_string()).unwrap();
    assert_eq!("tmp", file.get_name());
    assert_eq!(Some("/data/local".to_string()), file.get_parent());
    assert!(file.get_parent_file().is_some());
    assert!(file.is_absolute());
    assert_eq!("/data/local/tmp", file.get_absolute_path());
    assert!(file.get_canonical_path().is_ok());
    assert!(file.get_canonical_file().is_ok());
    assert!(file.can_read().is_ok());
    assert!(file.can_write().is_ok());
    assert!(file.exists().is_ok());
    assert!(file.is_directory().is_ok());
    assert!(file.is_file().is_ok());
    assert!(file.is_hidden().is_ok());
    assert!(file.last_modified().is_ok());
    assert!(file.length().is_ok());
    assert!(file.create_new_file().is_ok());
    assert!(file.delete().is_ok());
    assert!(file.delete_on_exit().is_ok());
    assert!(file.mkdir().is_ok());
    assert!(file.mkdirs().is_ok());
    assert!(file
        .rename_to(File::new("/data/local/tmp".to_string()).unwrap())
        .is_ok());
    assert!(file.set_last_modified(0).is_ok());
    assert!(file.set_read_only().is_ok());
    assert!(file.set_writable(false, false).is_ok());
    assert!(file.set_writable_convenience(false).is_ok());
    assert!(file.set_readable(false, false).is_ok());
    assert!(file.set_readable_convenience(false).is_ok());
    assert!(file.set_executable(false, false).is_ok());
    assert!(file.set_executable_convenience(false).is_ok());
    assert!(file.can_execute().is_ok());
    assert!(file.get_total_space().is_ok());
    assert!(file.get_free_space().is_ok());
    assert!(file.get_usable_space().is_ok());
    assert!(File::create_temp_file("droid".to_string(), None, None).is_ok());
    assert!(File::create_temp_file_default("droid".to_string(), None).is_ok());
    assert!(file.compare_to(&file).is_ok());
}
