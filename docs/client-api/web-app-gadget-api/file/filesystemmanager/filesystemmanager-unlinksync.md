---
document_id: '7073692582770049030'
directory_id: '7073451436034048005'
title: FileSystemManager.unlinkSync
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_unlink_sync
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.unlinkSync
document_type: GuideDocumentType
updated_at: 2022-11-07T08:13:01Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_unlink_sync
---

# FileSystemManager.unlinkSync

删除本地文件。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104"<br>path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"<br>fontSize="14">预览</md-preview-app> |


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| filePath | string | 是 |  | 要删除的文件路径 (本地路径) |



## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 代码示例

```js
const fileSystemManager = tt.getFileSystemManager();

tt.chooseImage({
  success(res) {
    // 保存临时文件到 用户目录
    const savedFilePath = fileSystemManager.saveFileSync(res.tempFilePaths[0]);

    try {
      // 删除文件
      fileSystemManager.unlinkSync(savedFilePath);
      console.log("删除成功");
    } catch (err) {
      console.log("删除失败", err);
    }
  },
});
```




