---
document_id: '7073692582769868806'
directory_id: '7073451436034048005'
title: FileSystemManager.readdirSync
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_dir_sync
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.readdirSync
document_type: GuideDocumentType
updated_at: 2022-12-02T02:53:36Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_dir_sync
---

# FileSystemManager.readdirSync

读取本地目录内文件列表。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app> |


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| dirPath | string | 是 |  | 要读取的目录路径 |


## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| files | Array<string\> | 指定目录下的文件名数组 |

  
## 代码示例

```js
const fileSystemManager = tt.getFileSystemManager();

try {
  const files = fileSystemManager.readdirSync("ttfile://user/");
  console.log("调用成功", files);
} catch (err) {
  console.log("调用失败", err);
}
```

