---
document_id: '7073693024736083973'
directory_id: '7073451436034048005'
title: FileSystemManager.accessSync
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/access_sync
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.accessSync
document_type: GuideDocumentType
updated_at: 2022-11-07T08:11:53Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/access_sync
---

# FileSystemManager.accessSync

判断文件/目录是否存在。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app> |


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| path | string | 是 |  | 要判断是否存在的文件/目录路径（包目录/临时目录/用户目录） |



## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


## 代码示例

```js
const fileSystemManager = tt.getFileSystemManager();
const path = `ttfile://temp/some_path`; // 判断的地址

try {
  fileSystemManager.accessSync(path);
  console.log(`${path} 地址存在`);
} catch (err) {
  console.log(`${path} 地址不存在或其他错误`, res.errMsg);
}
```
