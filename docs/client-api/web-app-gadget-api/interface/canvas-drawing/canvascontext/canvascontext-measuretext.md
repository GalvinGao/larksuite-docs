---
document_id: '7073692582769065990'
directory_id: '7073450228347256837'
title: CanvasContext.measureText
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-measureText
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.measureText
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-measureText
---

# CanvasContext.measureText(string text)

测量文字宽度

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.9.0+</md-version> | <md-version>V3.9.0+</md-version> | <md-version>V3.9.0+</md-version> | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| text | string | 是 |  | 需要测量的文字 |


## 输出

返回值：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| width | number | 文字宽度 |


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
    <md-preview-app type="gadget" disable="true" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::



```javascript
ctx.font = "italic bold 20px cursive";
const metrics = ctx.measureText("Hello Block");
console.log("MeasureText Result:", metrics.width);
```
