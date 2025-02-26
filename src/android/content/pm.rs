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

use crate::{JObjNew, JObjRef, JType, java_class};

/**
用于检索与设备上当前安装的应用程序包相关的各种信息的类。您可以通过 Context.getPackageManager 找到此类。
注意：如果您的应用以 Android 11（API 级别 30）或更高版本为目标，则此类中的方法均会返回经过筛选的应用列表。
*/
#[java_class(name = "android/content/pm/PackageManager")]
pub struct PackageManager;

impl PackageManager {
    /// 权限检查结果：如果权限已被授予给定的包，则由 checkPermission 返回此结果。
    pub const PERMISSION_GRANTED: i32 = 0;

    /// 权限检查结果：如果尚未授予给定包权限，则由 checkPermission 返回此结果。
    pub const PERMISSION_DENIED: i32 = -1;
}
