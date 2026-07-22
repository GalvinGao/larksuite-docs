---
document_id: '7073691561008627717'
directory_id: '7073451436034048005'
title: FileSystemManager.getSavedFileList
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_get_saved_file_list
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.getSavedFileList
document_type: GuideDocumentType
updated_at: 2022-11-07T08:12:06Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_get_saved_file_list
---

# FileSystemManager.getSavedFileList()

获取用户目录内文件列表。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app> |

## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性


## 输出

`success`返回对象参数的扩展属性：

名称 | 数据类型 | 描述
--|--|--|--|--
| fileList   | Array<FileItem\> | 文件数组，每一项是一个 `FileItem`


`FileItem` 的属性如下：

| 属性       | 类型   | 说明                                                          | 
| ---------- | ------ | ------------------------ | ------------ |
| filePath   | string | 文件路径                   |
| size       | number | 本地文件大小，以字节为单位    |
| createTime | number | 文件保存时的时间戳，从 `1970/01/01 08:00:00` 到当前时间的秒数 |

## 代码示例

```js
const fileSystemManager = getFileSystemManager();

fileSystemManager.getSavedFileList({
  success(res) {
    res.fileList.forEach((item) => {
      console.log(item.filePath, item.createTime, item.size);
    });
  },
  fail(res) {
    console.log("获取失败", res.errMsg);
  },
});
```
