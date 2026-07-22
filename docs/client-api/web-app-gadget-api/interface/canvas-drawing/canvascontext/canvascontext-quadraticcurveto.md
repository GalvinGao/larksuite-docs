---
document_id: '7073692582769704966'
directory_id: '7073450228347256837'
title: CanvasContext.quadraticCurveTo
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-quadraticCurveTo
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.quadraticCurveTo
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-quadraticCurveTo
---

# CanvasContext.quadraticCurveTo(number cpx, number cpy, number x, number y)

添加二次贝塞尔曲线到路径中

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| cpx | number | 是 |  | 控制点 x 坐标 |
| cpy | number | 是 |  | 控制点 y 坐标 |
| x | number | 是 |  | x 坐标 |
| y | number | 是 |  | y 坐标 |


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
// Quadratic Bézier curve
ctx.beginPath();
ctx.moveTo(50, 20);
ctx.quadraticCurveTo(230, 30, 50, 100);
ctx.stroke();

// Start and end points
ctx.fillStyle = "blue";
ctx.beginPath();
ctx.arc(50, 20, 5, 0, 2 * Math.PI);   // Start point
ctx.fill();
ctx.beginPath();
ctx.arc(50, 100, 5, 0, 2 * Math.PI);  // End point
ctx.fill();

// Control point
ctx.fillStyle = "red";
ctx.beginPath();
ctx.arc(230, 30, 5, 0, 2 * Math.PI);
ctx.fill();

ctx.draw();
```
