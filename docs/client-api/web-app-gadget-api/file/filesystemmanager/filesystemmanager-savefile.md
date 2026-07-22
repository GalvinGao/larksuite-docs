---
document_id: '7073692582769983494'
directory_id: '7073451436034048005'
title: FileSystemManager.saveFile
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_save_file
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.saveFile
document_type: GuideDocumentType
updated_at: 2022-11-07T08:12:45Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_save_file
---

# FileSystemManager.saveFile(Object object)

保存临时文件到本地永久目录

:::html
<md-alert type="tip">
该 API 会把临时文件移动到永久目录（目录最大200M），所以在调用成功后原文件路径将访问失败
</md-alert>
:::

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"<br>fontSize="14">预览</md-preview-app> |


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| tempFilePath | string | 是 |  | 文件临时路径<br>**示例值**：ttfile://temp/5c5fdd01-03bd-42cf-9938-31fb8b769a19-2863810ed1844e79f1b9bb880acb38d0.png |
| filePath | string | 否 |  | 文件路径。格式为：ttfile://user/feishu.png, 其中 ttfile://user/为固定格式，feishu.png 为文件名。如果不填则给一个随机路径。**不支持网络地址**<br>**示例值**：ttfile://user/feishu.png |


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| savedFilePath | string | 保存后的文件路径 |


## 示例代码

:::html

<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
    <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
    <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.chooseImage({
  success(res) {
    const tempFilePath = res.tempFilePaths[0];
    tt.saveFile({
      tempFilePath,
      filePath: "ttfile://user/feishu.png",
      success(res) {
        console.log(`${JSON.stringify(res)}`);
      },
      fail(res) {
        console.log(`saveFile fail: ${JSON.stringify(res)}`);
      },
    });
  },
});
```

`success`返回对象示例：

```json
{
    "savedFilePath": "ttfile://user/feishu.png",
    "errMsg": "saveFile:ok"
}
```
