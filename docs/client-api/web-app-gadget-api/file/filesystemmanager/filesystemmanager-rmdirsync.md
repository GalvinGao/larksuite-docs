---
document_id: '7073692582769295366'
directory_id: '7073451436034048005'
title: FileSystemManager.rmdirSync
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_rmdir_sync
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.rmdirSync
document_type: GuideDocumentType
updated_at: 2022-11-07T08:12:41Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_rmdir_sync
---

# FileSystemManager.rmdirSync

删除本地目录。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app> |


## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| dirPath | string | 是 |  | 本地目录路径 |
| recursive | boolean | 否 | false | 是否需要递归删除指定的目录 |


## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 代码示例


```js
const fileSystemManager = tt.getFileSystemManager();
// 必须以 "ttfile://user" 开头
const exmaplePath = "ttfile://user/example-dir";
try {
  fileSystemManager.mkdirSync(exmaplePath, false);
  console.log("成功");
} catch (err) {
  console.log("失败", err);
}
try {
  fileSystemManager.rmdirSync(exmaplePath);
  console.log("成功");
} catch (err) {
  console.log("失败", err);
}
```



