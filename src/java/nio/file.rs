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
    java::{io::File, lang::Comparable},
    JObjNew, JObjRef, JType,
};
use droid_wrap_derive::java_interface;

//noinspection SpellCheckingInspection
//noinspection GrazieInspection
/**
可用于在文件系统中定位文件的对象。它通常表示与系统相关的文件路径。
Path 表示分层路径，由由特殊分隔符或定界符分隔的目录和文件名元素序列组成。还可能存在标识文件系统层次结构的根组件。
距离目录层次结构的根最远的名称元素是文件或目录的名称。其他名称元素是目录名称。
Path 可以表示根、根和名称序列，或仅表示一个或多个名称元素。
如果 Path 仅由一个空的名称元素组成，则该 Path 被视为空路径。使用空路径访问文件相当于访问文件系统的默认目录。
Path 定义 getFileName、getParent、getRoot 和 subpath 方法来访问路径组件或其名称元素的子序列。
除了访问路径的组件之外，Path 还定义了 resolve 和 resolveSibling 方法来组合路径。 relativize 方法可用于构建两个路径之间的相对路径。
可以使用 startsWith 和 endsWith 方法比较和测试路径。
此接口扩展了 Watchable 接口，以便可以使用 WatchService 注册路径所在的目录并监视目录中的条目。
警告：此接口仅供开发自定义文件系统实现的人员实现。未来版本中可能会向此接口添加方法。
访问文件路径可以与 Files 类一起使用，以操作文件、目录和其他类型的文件。例如，假设我们希望 java.io.BufferedReader 从文件“access.log”中读取文本。
该文件位于相对于当前工作目录的目录“logs”中，并且是 UTF-8 编码的。
Path path = FileSystems.getDefault().getPath("logs", "access.log");
BufferedReader reader = Files.newBufferedReader(path, StandardCharsets.UTF_8);
互操作性与默认提供程序关联的路径通常可与 java.io.File 类互操作。其他提供程序创建的路径不太可能与 java.io.File 表示的抽象路径名互操作。
toPath 方法可用于从 java.io.File 对象表示的抽象路径名获取 Path。生成的 Path 可用于对与 java.io.File 对象相同的文件进行操作。
此外，toFile 方法可用于从 Path 的字符串表示构造 File。并发性此接口的实现是不可变的，并且可以安全地供多个并发线程使用。
*/
#[java_interface(name = "java/nio/file/Path")]
pub trait Path: JType + Comparable<Self>
where
    Self: Sized,
{
    /**
    指示此路径是否为绝对路径。绝对路径是完整的，因为它不需要与其他路径信息结合来定位文件。
    当且仅当此路径为绝对路径时，才返回 true
    */
    fn is_absolute(&self) -> bool;

    /**
    返回此路径的根组件作为“Path”对象，如果此路径没有根组件，则返回“null”。
    返回：表示此路径根组件的路径，或“null”
    */
    fn get_root<P: Path>(&self) -> Option<P>;

    /**
    返回该路径表示为“路径”对象的文件或目录的名称。文件名是距目录层次结构中的根最远的元素。
    返回：表示文件或目录名称的路径，或`null` 如果此路径为零元素
    */
    fn get_file_name<P: Path>(&self) -> Option<P>;

    /**
    返回 父路径，如果此路径没有父路径，则返回 null。
    此路径对象的父路径由此路径的根组件（如果有）和路径中的每个元素组成，但目录层次结构中距离根最远的元素除外。
    此方法不访问文件系统；路径或其父路径可能不存在。此外，此方法不会消除某些实现中可能使用的特殊名称，例如“.”和“..”。
    例如，在 UNIX 上，“/a/b/c”的父路径是“/a/b”，而“x/y/.”的父路径是“x/y”。
    此方法可以与 normalize 方法一起使用，以消除冗余名称，适用于需要类似 shell 的导航的情况。
    如果此路径有一个或多个元素，但没有根组件，则此方法相当于评估表达式： `subpath(0, getNameCount()-1);`
    返回：表示路径父路径的路径
    */
    fn get_parent<P: Path>(&self) -> Option<P>;

    /**
    返回路径中名称元素的数量。
    返回：路径中的元素数量，如果此路径仅代表根组件，则返回“0”
    */
    fn get_name_count(&self) -> i32;

    /**
    将此路径的名称元素作为 `Path` 对象返回。`index` 参数是要返回的名称元素的索引。
    目录层次结构中距离根最近的元素的索引为 `0`。距离根最远的元素的索引为 `getNameCount` count`-1`。
    返回：名称元素
    如果 `index` 为负数、`index` 大于或等于元素数，或者此路径有零个名称元素，则抛出：IllegalArgumentException
    `index` 元素的索引
    */
    fn get_name<P: Path>(&self, index: i32) -> Result<P, <Self as JType>::Error>;

    /**
    返回一个相对“Path”，它是此路径的名称元素的子序列。 “begin_index”和“end_index”参数指定名称元素的子序列。
    在目录层次结构中，最接近根的名称的索引为“0”。距离根最远的名称的索引为“getNameCount”count“-1”。
    返回的“Path”对象具有从“begin_index”开始并延伸到索引“end_index-1”处的元素的名称元素。
    返回：一个新的“Path”对象，它是此“Path”中名称元素的子序列
    抛出：IllegalArgumentException 如果“begin_index”为负数，或大于或等于元素数。如果“end_index”小于或等于“begin_index”，或大于元素数。
    `begin_index` 第一个元素的索引（包括）
    `end_index` 最后一个元素的索引（不包括）
    */
    fn subpath<P: Path>(
        &self,
        begin_index: i32,
        end_index: i32,
    ) -> Result<P, <Self as JType>::Error>;

    /**
    测试此路径是否以给定路径开头。如果此路径的根组件以给定路径的根组件开头，并且此路径以与给定路径相同的名称元素开头，则此路径以给定路径开头。
    如果给定路径的名称元素多于此路径，则返回 false。此路径的根组件是否以给定路径的根组件开头取决于文件系统。
    如果此路径没有根组件，而给定路径有根组件，则此路径不以给定路径开头。如果给定路径与此路径的其他 FileSystem 相关联，则返回 false。
    如果此路径以给定路径开头，则返回 true；否则返回 false
    `other` 给定路径
    */
    fn starts_with<P: Path>(&self, other: P) -> bool;

    /**
    测试此路径是否以 `Path` 开头，该路径是通过转换给定路径字符串构建的，其方式与 `startsWith(Path)` startsWith(Path) 方法指定的方式完全相同。
    例如，在 UNIX 上，路径“foo/bar”以“foo”和“foo/bar”开头。它不以“f”或“fo”开头。
    如果此路径以给定路径开头，则返回：`true`；否则返回`false`
    如果路径字符串无法转换为 Path，则抛出：InvalidPathException。
    `other` 给定的路径字符串
    */
    fn starts_with_string(&self, other: String) -> Result<bool, <Self as JType>::Error>;

    /**
    测试此路径是否以给定路径结束。如果给定路径有 N 个元素，没有根组件，并且此路径有 N 个或更多元素，则如果每条路径的最后 N 个元素（从距离根最远的元素开始）相等，则此路径以给定路径结束。
    如果给定路径有根组件，则如果此路径的根组件以给定路径的根组件结束，并且两个路径的相应元素相等，则此路径以给定路径结束。此路径的根组件是否以给定路径的根组件结束取决于文件系统。
    如果此路径没有根组件，而给定路径有根组件，则此路径不以给定路径结束。如果给定路径与此路径的不同 FileSystem 相关联，则返回 false。
    返回：如果此路径以给定路径结束，则返回 true；否则返回 false
    `other` 给定路径
    */
    fn ends_with<P: Path>(&self, other: P) -> bool;

    /**
    测试此路径是否以 `Path` 结尾，该路径通过转换给定的路径字符串构造而成，其方式与 `endsWith(Path)` endsWith(Path) 方法指定的方式完全相同。
    例如，在 UNIX 上，路径“foo/bar”以“foo/bar”和“bar”结尾。它不以“r”或“/bar”结尾。
    请注意，不考虑尾随分隔符，因此在 `Path`“foo/bar” 上使用 `String`“bar/”调用此方法将返回 `true`。
    如果此路径以给定的路径结尾，则返回：`true`；否则返回 `false`
    如果路径字符串无法转换为 Path，则抛出：InvalidPathException。
    `other` 给定的路径字符串
    */
    fn ends_with_string(&self, other: String) -> Result<bool, <Self as JType>::Error>;

    /**
    返回一条路径，该路径是此路径，但消除了冗余名称元素。此方法的精确定义取决于实现，但通常它源自此路径，即不包含冗余名称元素的路径。
    在许多文件系统中，“.”和“..”是用于指示当前目录和父目录的特殊名称。
    在此类文件系统中，所有出现的“.”都被视为冗余。如果“..”前面有一个非“..”名称，则这两个名称都被视为冗余（重复识别此类名称的过程，直到不再适用为止）。
    此方法不访问文件系统；路径可能无法找到存在的文件。从路径中消除“..”和前面的名称可能会导致路径找到与原始路径不同的文件。当前面的名称是符号链接时可能会出现这种情况。
    返回：结果路径或此路径（如果它不包含冗余名称元素）；如果此路径确实有根组件并且所有名称元素都是冗余的，则返回空路径
    */
    fn normalize<P: Path>(&self) -> P;

    // -- 解析和相对化 --

    /**
    根据此路径解析给定路径。如果其他参数是`isAbsolute()`绝对路径，则此方法通常会返回`other`。如果`other`是空路径，则此方法通常会返回此路径。
    否则，此方法将此路径视为目录，并根据此路径解析给定路径。在最简单的情况下，给定路径没有`getRoot`根组件，在这种情况下，此方法将给定路径连接到此路径并返回以给定路径`endsWith`结尾的结果路径。
    如果给定路径具有根组件，则解析高度依赖于实现，因此未指定。
    返回：结果路径
    `other` 根据此路径解析的路径
    */
    fn resolve<P: Path>(&self, other: P) -> P;

    /**
    将给定的路径字符串转换为 `Path`，并按照 `resolve(Path)` 解析方法指定的方式根据此 `Path` 进行解析。
    例如，假设名称分隔符为“`/`”，路径表示“`foo/bar`”，则使用路径字符串“`gus`”调用此方法将得到 `Path`“`foo/bar/gus`”。
    返回：结果路径
    如果路径字符串无法转换为 Path，则抛出：InvalidPathException。
    `other` 根据此路径进行解析的路径字符串
    */
    fn resolve_string<P: Path>(&self, other: String) -> Result<P, <Self as JType>::Error>;

    /**
    根据此路径的 `getParent` 父路径解析给定路径。当需要用另一个文件名替换文件名时，这很有用。
    例如，假设名称分隔符为“`/`”，路径表示“`dir1/dir2/foo`”，则使用 `Path` “`bar`” 调用此方法将得到 `Path` “`dir1/dir2/bar`”。如果此路径没有父路径，或者 `other` 是 `isAbsolute()` 绝对路径，则此方法返回 `other`。
    如果 `other` 是空路径，则此方法返回此路径的父路径，或者如果此路径没有父路径，则返回空路径。
    返回：结果路径
    `other` 根据此路径的父路径解析的路径
    */
    fn resolve_sibling<P: Path>(&self, other: P) -> P;

    /**
    将给定的路径字符串转换为 `Path`，并按照 `resolveSibling(Path)` resolveSibling 方法指定的方式根据此路径的 `getParent` 父路径进行解析。
    返回：结果路径
    如果路径字符串无法转换为 Path，则抛出：InvalidPathException。
    `other` 根据此路径的父路径进行解析的路径字符串
    */
    fn resolve_sibling_string<P: Path>(&self, other: String) -> Result<P, <Self as JType>::Error>;

    /**
    构造此路径与给定路径之间的相对路径。相对化是 `resolve(Path)` 解析的逆过程。
    此方法尝试构造一个 `isAbsolute` 相对路径，当针对此路径解析 `resolve(Path)` 时，将生成一个与给定路径定位相同文件的路径。
    例如，在 UNIX 上，如果此路径为 `"/a/b"` 且给定路径为 `"/a/b/c/d"`，则生成的相对路径将为 `"c/d"`。
    如果此路径和给定路径没有 `getRoot` 根组件，则可以构造相对路径。如果只有一条路径具有根组件，则无法构造相对路径。
    如果两条路径都有根组件，则是否可以构造相对路径取决于实现。如果此路径和给定路径 `equals` 相等，则返回空路径。
    对于任何两个 `normalize` 规范化路径 p 和 q，其中 q 没有根组件，`p.relativize(p.resolve(q)).equals(q)` 当支持符号链接时，结果路径在根据此路径解析时是否产生可用于定位 `Files#isSameFile` 与 `other` 相同的文件的路径取决于实现。
    例如，如果此路径为 `"/a/b"` 且给定路径为 `"/a/x"`，则结果相对路径可能为 `"../x"`。如果 `"b"` 是符号链接，则 `"a/b/../x"` 是否会定位与 `"/a/x"` 相同的文件取决于实现。
    返回：生成的相对路径，如果两个路径相等，则返回空路径。
    如果 `other` 不是可以相对于此路径进行相对化的 `Path`，则抛出：IllegalArgumentException
    `other` 相对于此路径进行相对化的路径
    */
    fn relativize<P: Path>(&self, other: P) -> Result<P, <Self as JType>::Error>;

    /**
    返回表示此路径绝对路径的 `Path` 对象。如果此路径已经是 `Path#isAbsolute` 绝对路径，则此方法仅返回此路径。
    否则，此方法以依赖于实现的方式解析路径，通常通过根据文件系统默认目录解析路径。根据实现，如果文件系统不可访问，此方法可能会抛出 I/O 错误。
    返回：表示绝对路径的 `Path` 对象
    抛出：
    - java.io.IOError 如果发生 I/O 错误；
    - SecurityException 在默认提供程序的情况下，安装了安全管理器，并且此路径不是绝对路径，则调用安全管理器的 `SecurityManager#checkPropertyAccess(String)` checkPropertyAccess 方法来检查对系统属性 `user.dir` 的访问权限
    */
    fn to_absolute_path<P: Path>(&self) -> Result<P, <Self as JType>::Error>;

    /**
    返回表示此路径的 `File` 对象。如果此 `Path` 与默认提供程序相关联，则此方法相当于返回使用此路径的 `String` 表示形式构造的 `File` 对象。
    如果此路径是通过调用 `File` `File#toPath` toPath 方法创建的，则无法保证此方法返回的 `File` 对象与原始 `File` 相等。
    返回：表示此路径的 `File` 对象
    抛出：如果此 `Path` 与默认提供程序不相关联，则抛出 UnsupportedOperationException
    */
    fn to_file(&self) -> Result<File, <Self as JType>::Error>;
}
