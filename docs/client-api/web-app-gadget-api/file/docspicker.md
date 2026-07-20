---
document_id: '6965379543683432454'
directory_id: '6907567269107810306'
title: docsPicker
full_path: /uYjL24iN/ukTN3UjL5UzN14SO1cTN
breadcrumb:
- Client API
- Web app/Gadget API
- File
- docsPicker
document_type: GuideDocumentType
updated_at: 2022-11-07T08:11:44Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukTN3UjL5UzN14SO1cTN
---

# docsPicker(Object object)

打开云文档选择列表

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
<md-td><md-version>V3.12.0+</md-version></md-td>
<md-td><md-version>V3.12.0+</md-version></md-td>
<md-td><md-version>V3.13.0+</md-version></md-td>
<md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
<md-tr>
<md-td>网页应用</md-td>
<md-td><md-version>V3.44.0+</md-version></md-td>
<md-td><md-version>V3.44.0+</md-version></md-td>
<md-td><md-version>V3.47.0+</md-version></md-td>
<md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app></md-td>
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
10
</md-td>
<md-td>
最大文件选择数量
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
Select Docs
</md-td>
<md-td>
允许开发者自定义组件的标题文案
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
Select
</md-td>
<md-td>
允许开发者自定义组件的选择按钮文案
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
fileList
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
filePath
</md-text>
</md-td>
<md-td>
string
</md-td>
<md-td>
docs 文件 url
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
fileName
</md-text>
</md-td>
<md-td>
string
</md-td>
<md-td>
docs 文件名
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
    <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.docsPicker({
  maxNum: 5,
  pickerTitle: "Select Docs",
  pickerConfirm: "Select",
  success(res) {
    console.log(JSON.stringify(res));
  },
  fail(res) {
    console.log(`docsPicker fail: ${JSON.stringify(res)}`);
  },
});
```

`success`返回对象示例：

```json
{
  "errMsg": "docsPicker:ok",
  "fileList": [
    {
      "filePath": "https://bytedance.feishu.cn/base/bascn**************",
      "fileName": "示例文档1"
    },
    {
      "filePath": "https://bytedance.feishu.cn/docx/doxcn**************",
      "fileName": "示例文档2"
    }
  ]
}
```
