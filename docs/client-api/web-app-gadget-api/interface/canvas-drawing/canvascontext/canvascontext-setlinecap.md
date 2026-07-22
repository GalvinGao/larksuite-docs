---
document_id: '7073692582769311750'
directory_id: '7073450228347256837'
title: CanvasContext.setLineCap
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineCap
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.setLineCap
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineCap
---

# CanvasContext.setLineCap(string cap)

设置线端点样式

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| cap | string | 是 |  | 线端点样式<br>**可选值**：<br>- `butt` 默认样式，端点为方形<br>- `round` 端点为弧形，圆心为端点<br>- `square` 端点为覆盖弧形的最小方形 |


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
const lineCap = ["butt", "round", "square"];

ctx.strokeStyle = "blue";
ctx.beginPath();
ctx.moveTo(10, 10);
ctx.lineTo(140, 10);
ctx.moveTo(10, 140);
ctx.lineTo(140, 140);
ctx.stroke();

ctx.strokeStyle = "black";
for (let i = 0; i < lineCap.length; ++i) {
  ctx.lineWidth = 15;
  ctx.setLineCap(lineCap[i]);
  ctx.beginPath();
  ctx.moveTo(25 + i * 50, 10);
  ctx.lineTo(25 + i * 50, 140);
  ctx.stroke();
}

ctx.draw();
```
