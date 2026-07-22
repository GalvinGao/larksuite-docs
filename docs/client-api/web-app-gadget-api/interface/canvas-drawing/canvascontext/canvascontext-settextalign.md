---
document_id: '7073692582769442822'
directory_id: '7073450228347256837'
title: CanvasContext.setTextAlign
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTextAlign
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.setTextAlign
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTextAlign
---

# CanvasContext.setTextAlign(string align)

设置字体对齐方式

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| align | string | 是 |  | 对齐方式<br>**可选值**：<br>- `left` 左对齐<br>- `right` 右对齐<br>- `center` 居中对齐<br>- `start` 按照书写习惯的开始方向对齐<br>- `end` 按照书写习惯的结束方向对齐 |


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
const textAlignList = ["left", "center", "right"];
ctx.strokeStyle = "red";
ctx.moveTo(150, 20);
ctx.lineTo(150, 170);
ctx.stroke();
ctx.setFontSize(15);

for (let i = 0; i < textAlignList.length; ++i) {
  ctx.setTextAlign(textAlignList[i]);
  ctx.fillText(`textAlign = ${textAlignList[i]}`, 150, 40 + 20 * i);
}

ctx.draw();
```
