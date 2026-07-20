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

:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width: 20%;">应用能力</md-th>
<md-th style="width: 20%;">Android</md-th>
<md-th style="width: 20%;">iOS</md-th>
<md-th style="width: 20%;">PC</md-th>
<md-th style="width: 20%;">预览效果</md-th>
</md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>小程序</md-td>
<md-td>**✓**</md-td>
<md-td>**✓**</md-td>
<md-td>**✓**</md-td>
<md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
<md-tr>
<md-td>网页应用</md-td>
<md-td><md-version>V3.44.0+</md-version></md-td>
<md-td><md-version>V3.44.0+</md-version></md-td>
<md-td><md-version>V3.47.0+</md-version></md-td>
<md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
</md-tbody>
</md-table>
:::

## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width: 20%;">
名称
</md-th>
<md-th style="width: 18%;">
数据类型
</md-th>
<md-th style="width: 10%;">
必填
</md-th>
<md-th style="width: 10%;">
默认值
</md-th>
<md-th>
描述
</md-th>
</md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>
maxNum
</md-td>
<md-td>
number
</md-td>
<md-td>
否
</md-td>
<md-td>
</md-td>
<md-td>
最大选择数量

**示例值**：10

</md-td>
</md-tr>
<md-tr>
<md-td>
isSystem
</md-td>
<md-td>
boolean
</md-td>
<md-td>
否
</md-td>
<md-td>
false
</md-td>
<md-td>
                是否使用系统的文件选择器

**可选值**：
- `true`：使用系统的文件选择器，可以选择系统文件系统里的文件，并且 maxNum 参数会被设置为 1
- `false`：使用Lark的文件选择器，可以选择Lark文档

<md-alert type="tip" icon="none">
- Android/iOS 端：Lark[V3.8.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- PC 端：暂不支持

</md-alert>
  </md-td>
  </md-tr>
  <md-tr>
  <md-td>
  pickerTitle
  </md-td>
  <md-td>
  string
  </md-td>
  <md-td>
  否
  </md-td>
  <md-td>
  Select a file
  </md-td>
  <md-td>
  允许开发者自定义文件选择器标题，仅在Lark文件选择器生效
  <md-alert type="tip" icon="none">
  Lark[V3.37.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
  </md-alert>  
  </md-td>
  </md-tr>
  <md-tr>
  <md-td>
  pickerConfirm
  </md-td>
  <md-td>
  string
  </md-td>
  <md-td>
  否
  </md-td>
  <md-td>
  Confirm
  </md-td>
  <md-td>
  允许开发者自定义组件的选择按钮文案，仅在Lark文件选择器下生效
  <md-alert type="tip" icon="none">
- Android/iOS 端：Lark[V3.37.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- PC 端：Lark[V3.37](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)-[V3.40](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)版本支持
  </md-alert>
  </md-td>
  </md-tr>
  </md-tbody>
  </md-table>
  :::

## 输出

`success`返回对象的扩展属性：

:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width: 30%;">
名称
</md-th>
<md-th style="width: 18%;">
数据类型
</md-th>
<md-th>
描述
</md-th>
</md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>
list
</md-td>
<md-td>
object[]
</md-td>
<md-td>
文件列表
</md-td>
</md-tr>
<md-tr>
<md-td>
&emsp;
<span style="color: #8F959E">
∟
</span>
&nbsp;
<md-text type="field-name">
path
</md-text>
</md-td>
<md-td>
string
</md-td>
<md-td>
文件路径
</md-td>
</md-tr>
<md-tr>
<md-td>
&emsp;
<span style="color: #8F959E">
∟
</span>
&nbsp;
<md-text type="field-name">
name
</md-text>
</md-td>
<md-td>
string
</md-td>
<md-td>
文件名
</md-td>
</md-tr>
<md-tr>
<md-td>
&emsp;
<span style="color: #8F959E">
∟
</span>
&nbsp;
<md-text type="field-name">
size
</md-text>
</md-td>
<md-td>
string
</md-td>
<md-td>
文件大小
</md-td>
</md-tr>
</md-tbody>
</md-table>
:::

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
