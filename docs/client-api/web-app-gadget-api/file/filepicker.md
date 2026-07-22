---
document_id: '6965379541104984069'
directory_id: '6907567269107810306'
title: filePicker
full_path: /uYjL24iN/uETM04SMxQjLxEDN
breadcrumb:
- Client API
- Web app/Gadget API
- File
- filePicker
document_type: GuideDocumentType
updated_at: 2022-11-07T08:11:41Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETM04SMxQjLxEDN
---

# filePicker(Object object)

打开附件选择列表

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.47.0+</md-version> | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| maxNum | number | 否 |  | 最大选择数量<br>**示例值**：10 |
| isSystem | boolean | 否 | false | 是否使用系统的文件选择器<br>**可选值**：<br>- `true`：使用系统的文件选择器，可以选择系统文件系统里的文件，并且 maxNum 参数会被设置为 1<br>- `false`：使用Lark的文件选择器，可以选择Lark文档<br><md-alert type="tip" icon="none"><br>- Android/iOS 端：Lark[V3.8.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- PC 端：暂不支持<br></md-alert> |
| pickerTitle | string | 否 | Select a file | 允许开发者自定义文件选择器标题，仅在Lark文件选择器生效<br><md-alert type="tip" icon="none"><br>Lark[V3.37.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| pickerConfirm | string | 否 | Confirm | 允许开发者自定义组件的选择按钮文案，仅在Lark文件选择器下生效<br><md-alert type="tip" icon="none"><br>- Android/iOS 端：Lark[V3.37.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- PC 端：Lark[V3.37](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)-[V3.40](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)版本支持<br></md-alert> |


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| list | object[] | 文件列表 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>path<br></md-text> | string | 文件路径 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>name<br></md-text> | string | 文件名 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>size<br></md-text> | string | 文件大小 |


## 示例代码

:::html

<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>
  <div style="display: flex">
    <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="16" style="margin-right: 24px">预览小程序 </md-preview-app>
    <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.filePicker({
  maxNum: 10,
  pickerTitle: "Select a file",
  pickerConfirm: "Confirm",
  isSystem: false,
  success(res) {
    console.log(JSON.stringify(res));
  },
  fail(res) {
    console.log(`filePicker fail: ${JSON.stringify(res)}`);
  },
});
```

`success`返回对象示例：

```json
{
  "errMsg": "filePicker:ok",
  "list": [
    {
      "path": "ttfile://temp/311d60f2-9a56-4f43-8758-92aa7378fad8-app.js",
      "name": "app.js",
      "size": 40
    }
  ]
}
```
