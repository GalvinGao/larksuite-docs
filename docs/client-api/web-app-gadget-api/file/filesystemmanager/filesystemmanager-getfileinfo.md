---
document_id: '7073692582769049606'
directory_id: '7073451436034048005'
title: FileSystemManager.getFileInfo
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_get_file_info
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.getFileInfo
document_type: GuideDocumentType
updated_at: 2022-11-07T08:12:03Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_get_file_info
---

# FileSystemManager.getFileInfo(Object object)

获取本地文件信息。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app> |


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| filePath | string | 是 |  | 本地文件路径 |


## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| size | number | 文件大小，以字节为单位 |


## 代码示例

```js
const fileSystemManager = tt.getFileSystemManager();

tt.chooseImage({
  success(res) {
    getFileInfo(res.tempFilePaths[0]);
  },
});

function getFileInfo(filePath) {
  fileSystemManager({
    filePath,
    success(res) {
      console.log("文件信息:", res);
    },
    fail(res) {
      console.log("调用失败", res.errMsg);
    },
  });
}
```

