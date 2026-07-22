---
document_id: '7162040901753438213'
directory_id: '7073451436034048005'
title: FileSystemManager.unzip
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_unzip
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.unzip
document_type: GuideDocumentType
updated_at: 2022-11-07T08:13:10Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_unzip
---

# FileSystemManager.unzip(Object object)

解压zip文件到用户目录。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104"<br>path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"<br>fontSize="14">预览</md-preview-app> |


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| zipFilePath | string | 是 |  | 源文件路径，只可以是 zip 压缩文件 |
| targetPath | string | 是 |  | 目标目录路径, 必须以 `ttfile://user` 开头 |



## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 代码示例

```js
const fileSystemManager = tt.getFileSystemManager();

// 下载一个 zip 文件
tt.downloadFile({
  url:
    "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/973055dc36ba30b1ab350654c15a4b31.zip",
  success(res) {
    console.log("下载 zip 成功", res.tempFilePath);
    // 解压 zip 文件
    unzip(res.tempFilePath);
  },
  fail(res) {
    console.log("下载 zip 失败", res.errMsg);
  },
});

function unzip(zipFilePath) {
  const targetPath = `ttfile://user/abc`;

  fileSystemManager.unzip({
    zipFilePath,
    targetPath,
    success(_res) {
      console.log("解压成功");
      const concent = fs.readdirSync(targetPath);
      // 输出 zip 内文件内容
      console.log("压缩包内文件:", concent);
    },
    fail(res) {
      console.log("解压失败", res.errMsg);
    },
  });
}
```

