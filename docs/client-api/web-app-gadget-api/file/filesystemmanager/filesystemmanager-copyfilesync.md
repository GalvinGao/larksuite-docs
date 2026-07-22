---
document_id: '7073691561008545797'
directory_id: '7073451436034048005'
title: FileSystemManager.copyFileSync
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_copy_file_sync
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.copyFileSync
document_type: GuideDocumentType
updated_at: 2022-11-07T08:12:00Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_copy_file_sync
---

# FileSystemManager.copyFileSync

复制本地文件/目录。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app> |


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| srcPath | string | 是 |  | 源文件路径 |
| destPath | string | 是 |  | 目标文件路径 |


## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


## 代码示例

```js
const fileSystemManager = tt.getFileSystemManager();
const srcPath = `ttfile://temp/some_path`;
const destPath = `ttfile://temp/some_path_copy`;

try {
  // 拷贝文件, destPath 目录必须以 `ttfile://user` 开头
  fileSystemManager.copyFileSync(
    srcPath,
    destPath
  );
  console.log("拷贝成功");
} catch (err) {
  console.log("拷贝失败", err);
}
```

