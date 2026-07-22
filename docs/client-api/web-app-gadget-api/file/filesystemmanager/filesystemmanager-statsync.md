---
document_id: '7073691561008644101'
directory_id: '7073451436034048005'
title: FileSystemManager.statSync
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_stat_sync
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.statSync
document_type: GuideDocumentType
updated_at: 2022-11-07T08:12:54Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_stat_sync
---

# FileSystemManager.statSync

获取本地文件 Stats 对象。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104"<br>path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"<br>fontSize="14">预览</md-preview-app> |


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| path | string | 是 |  | 本地文件路径<br>**示例值**：ttfile://temp |


## 输出

返回对象的扩展属性与方法：
:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| stat | object | Stats 对象 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>mode<br></md-text> | number | 文件的类型和存取的权限，对应 POSIX stat.st_mode |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>size<br></md-text> | number | 文件大小，单位：B，对应 POSIX stat.st_size |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>lastAccessedTime<br></md-text> | number | 文件最近一次被存取或被执行的时间，UNIX 时间戳，对应 POSIX stat.st_atime |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>lastModifiedTime<br></md-text> | number | 文件最后一次被修改的时间，UNIX 时间戳，对应 POSIX stat.st_mtime |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[isDirectory()](/document/uYjL24iN/uETOuETOuETO/stat/stats_is_directory)<br></md-text> | function | 判断当前文件是否一个目录 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[isFile()](/document/uYjL24iN/uETOuETOuETO/stat/stats_is_file)<br></md-text> | function | 判断当前文件是否一个普通文件 |




## 代码示例

```js
const fileSystemManager = tt.getFileSystemManager();

tt.chooseImage({
  success(res) {
    const tempFile = res.tempFilePaths[0];
    try {
      const stat = fileSystemManager.statSync(tempFile);
      console.log("是否是目录:", res.stat.isDirectory());
      console.log("是否是文件:", res.stat.isFile());
    } catch (err) {
      console.log("调用失败", err);
    }
  },
});
```



