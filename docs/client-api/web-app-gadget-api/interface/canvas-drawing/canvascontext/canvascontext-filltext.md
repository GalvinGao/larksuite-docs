---
document_id: '7073693024735281157'
directory_id: '7073450228347256837'
title: CanvasContext.fillText
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fillText
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.fillText
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fillText
---

# CanvasContext.fillText(string text, number x, number y, number maxWidth)

填充文字

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| text | string | 是 |  | 文字 |
| x | number | 是 |  | y 坐标 |
| y | number | 是 |  | y 坐标 |
| maxWidth | number | 否 |  | 文字最大宽度 |


## 输出

无

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
const ctx = tt.createCanvasContext(canvasId);

ctx.setFontSize(20);
ctx.fillText("Hello Block!", 20, 20);

ctx.draw();
```
