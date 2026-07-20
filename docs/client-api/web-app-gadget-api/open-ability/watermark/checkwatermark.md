---
document_id: '6965379541104689157'
directory_id: '6907567266541256706'
title: checkWatermark
full_path: /uYjL24iN/ukTM1EjL5ETNx4SOxUTM
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Watermark
- checkWatermark
document_type: GuideDocumentType
updated_at: 2022-03-11T04:13:46Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukTM1EjL5ETNx4SOxUTM
---

# checkWatermark(Object object)

查看宿主是否显示了全局水印。

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
<md-td><md-version>V2.7.0+</md-version></md-td>
<md-td><md-version>V2.7.0+</md-version></md-td>
<md-td><md-version>V2.7.0+</md-version></md-td>
<md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/watermark/watermark" fontSize="14">预览</md-preview-app>
</md-td>
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

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性

## 输出

`success`返回对象的扩展属性：

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
<md-th>
描述
</md-th>
</md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>
hasWatermark
</md-td>
<md-td>
boolean
</md-td>
<md-td>
宿主是否显示了全局水印
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
    <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/watermark/watermark" fontSize="16" style="margin-right: 24px">预览小程序 </md-preview-app>
    <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.checkWatermark({
  success(res) {
    console.log(JSON.stringify(res));
  },
  fail(res) {
    console.log(`checkWatermark fail: ${JSON.stringify(res)}`);
  },
});
```

`success`返回对象示例：

```json
{
  "errMsg": "checkWatermark:ok",
  "hasWatermark": true
}
```
