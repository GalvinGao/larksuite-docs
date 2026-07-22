---
document_id: '7073692582769033222'
directory_id: '7073450228347256837'
title: CanvasContext.setLineJoin
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineJoin
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.setLineJoin
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineJoin
---

# CanvasContext.setLineJoin(string join)

设置线连接点样式

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| join | string | 是 |  | 线连接点样式<br>**可选值**：<br>- `bevel` 类似于弧形，但是用直线连接<br>- `round` 弧形连接<br>- `miter` 默认样式，正常连接 |


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
const lineJoin = ["round", "bevel", "miter"];
ctx.lineWidth = 10;

for (let i = 0; i < lineJoin.length; ++i) {
  ctx.setLineJoin(lineJoin[i]);
  ctx.beginPath();
  ctx.moveTo(10, 10 + i * 40);
  ctx.lineTo(50, 50 + i * 40);
  ctx.lineTo(90, 10 + i * 40);
  ctx.lineTo(130, 50 + i * 40);
  ctx.lineTo(170, 10 + i * 40);
  ctx.stroke();
}

ctx.draw();
```
