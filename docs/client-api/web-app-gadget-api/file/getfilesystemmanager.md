---
document_id: '7073691561007939589'
directory_id: '6907567269107810306'
title: getFileSystemManager
full_path: /uYjL24iN/uETOuETOuETO/tt_get_file_system_manager
breadcrumb:
- Client API
- Web app/Gadget API
- File
- getFileSystemManager
document_type: GuideDocumentType
updated_at: 2022-11-07T08:11:47Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/tt_get_file_system_manager
---

# getFileSystemManager()

获取全局唯一的文件管理器。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V4.11.0+</md-version> | <md-version>V4.11.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V4.11.0+</md-version> | <md-version>V4.11.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14" disable="true">预览</md-preview-app> |


## 目录说明

小程序文件系统分为以下 3 个部分

### 用户目录

以 `ttfile://user` 开头，开发者拥有读写权限

### 临时目录

以 `ttfile://temp` 开头，开发者拥有读权限，删权限，无写权限

### 包目录

小程序包体目录，开发者拥有读权限，无删权限，无写权限


包目录路径支持格式包括 `a/b/c`, `/a/b/c`, `./a/b/c` 且仅支持这三种格式。 例如读取包内 `app.js` 文件。

```js

const fileSystemManager = tt.getFileSystemManager();

fileSystemManager.readFile({
  filePath: "app.js",// "/app.js", "./app.js"
  encoding: "utf8",
  success(res) {
    // app.js 文件内容
    console.log(res.data);
  },
  fail(res) {
    console.error("读取失败", res.errMsg);
  },
});

```

## 输入
无


## 输出

返回值：`FileSystemManager`，该对象的方法列表参见下表：

:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::

| 方法 | 介绍 |
| --- | --- |
| [access(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_access) | 判断本地文件/目录是否存在 |
| [accessSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/access_sync) | 同步判断本地文件文件/目录是否存在 |
| [saveFile(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_save_file) | 保存临时文件到用户目录 |
| [saveFileSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_save_file_sync) | 同步保存临时文件到用户目录 |
| [getFileInfo(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_get_file_info) | 获取本地文件信息 |
| [getSavedFileList()](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_get_saved_file_list) | 获取用户目录文件列表 |
| [removeSavedFile(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_remove_saved_file) | 删除用户目录文件 |
| [copyFile(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_copy_file) | 复制本地文件 |
| [copyFileSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_copy_file_sync) | 同步复制本地文件 |
| [mkdir(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_mkdir) | 创建本地目录 |
| [mkdirSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_mkdir_sync) | 同步创建本地目录 |
| [readdir(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_dir) | 读取本地目录内文件列表 |
| [readdirSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_dir_sync) | 同步读取本地目录内文件列表 |
| [readFile(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_file) | 读取本地文件内容 |
| [readFileSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_file_sync) | 同步读取本地文件内容 |
| [rename(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_rename) | 重命名本地文件 |
| [renameSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_rename_sync) | 同步重命名本地文件 |
| [rmdir(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_rmdir) | 删除本地目录 |
| [rmdirSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_rmdir_sync) | 同步删除本地目录 |
| [stat(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_stat) | 获取本地文件 Stats 对象 |
| [statSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_stat_sync) | 同步获取本地文件 Stats 对象 |
| [unlink(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_unlink) | 删除本地文件 |
| [unlinkSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_unlink_sync) | 同步删除本地文件 |
| [writeFile(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_write_file) | 写入本地文件 |
| [writeFileSync](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_write_file_sync) | 同步写入本地文件 |
| [unzip(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_unzip) | 解压zip文件到用户目录。 |

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
        <md-preview-app type="webApp"  fontSize="16" disable="true">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
try {
    let result = tt.getFileSystemManager();
    console.log(`getFileSystemManager success: ${JSON.stringify(result)}`);
} catch (error) {
    console.log(`getFileSystemManager fail: ${JSON.stringify(error)}`);
}
```


