---
document_id: '7073693024735346693'
directory_id: '7073451436034048005'
title: FileSystemManager.mkdirSync
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_mkdir_sync
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.mkdirSync
document_type: GuideDocumentType
updated_at: 2022-11-07T08:12:13Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_mkdir_sync
---

# FileSystemManager.mkdirSync

创建本地目录。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app> |


## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| dirPath | string | 是 |  | 创建的目录路径, 必须 ttfile://user 开头 |
| recursive | boolean | 否 | false | 是否递归创建该目录的上级目录后再创建该目录。 |


## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 代码示例


```js
const fileSystemManager = tt.getFileSystemManager();

try {
  fileSystemManager.mkdirSync("ttfile://user/some/path", true);
  console.log("调用成功");
} catch (err) {
  console.log("调用失败", err);
}
```

