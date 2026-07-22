---
document_id: '7073693024735395845'
directory_id: '7073450228347256837'
title: CanvasContext.bezierCurveTo
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-bezierCurveTo
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.bezierCurveTo
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-bezierCurveTo
---

# CanvasContext.bezierCurveTo(number cp1x, number cp1y, number cp2x, number cp2y, number x, number y)

添加三次贝塞尔曲线到路径中

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| cp1x | number | 是 |  | 控制点1 x 坐标 |
| cp1y | number | 是 |  | 控制点1 y 坐标 |
| cp2x | number | 是 |  | 控制点2 x 坐标 |
| cp2y | number | 是 |  | 控制点2 y 坐标 |
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
ctx.beginPath();
ctx.moveTo(50, 20);
ctx.bezierCurveTo(230, 30, 150, 60, 50, 100);
ctx.stroke();

ctx.fillStyle = "blue";
// start point
ctx.fillRect(50, 20, 10, 10);
// end point
ctx.fillRect(50, 100, 10, 10);

ctx.fillStyle = "red";
// control point one
ctx.fillRect(230, 30, 10, 10);
// control point two
ctx.fillRect(150, 70, 10, 10);

ctx.draw();
```
