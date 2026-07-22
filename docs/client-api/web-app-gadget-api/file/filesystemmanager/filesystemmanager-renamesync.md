---
document_id: '7073693024735592453'
directory_id: '7073451436034048005'
title: FileSystemManager.renameSync
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_rename_sync
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.renameSync
document_type: GuideDocumentType
updated_at: 2022-11-07T08:12:35Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_rename_sync
---

# FileSystemManager.renameSync

重命名本地文件/目录


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app> |


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| oldPath | string | 是 |  | 源文件路径 |
| newPath | string | 是 |  | 新文件路径 |



## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


## 代码示例


```js
// 将所有保存的文件移除拓展标识
const fileSystemManager = tt.getFileSystemManager();

fileSystemManager.getSavedFileList({
  success(res) {
    res.fileList.forEach(removeExt);
  },
  fail(res) {
    console.log("获取失败", res.errMsg);
  },
});

function removeExt(fileItem) {
  console.log(`移除 ${fileItem.filePath} 的 ext`);
  const newPath = fileItem.filePath.replace(/(\..+)?$/, "");

  try {
    fileSystemManager.renameSync(fileItem.filePath, newPath);
    console.log("重命名成功");
  } catch (err) {
    console.log("重命名失败", err);
  }
}
```
