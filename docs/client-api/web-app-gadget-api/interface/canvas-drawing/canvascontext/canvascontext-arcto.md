---
document_id: '7073691561008283653'
directory_id: '7073450228347256837'
title: CanvasContext.arcTo
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-arcTo
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.arcTo
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-arcTo
---

# CanvasContext.arcTo(number x1, number y1, number x2, number y2, number radius)

移动并添加弧线到当前路径中

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| x1 | number | 是 |  | 控制点1 x 坐标 |
| y1 | number | 是 |  | 控制点1 y 坐标 |
| x2 | number | 是 |  | 控制点2 x 坐标 |
| y2 | number | 是 |  | 控制点2 y 坐标 |
| radius | number | 是 |  | x 坐标 |


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
ctx.fillRect(100, 100, 5, 5);
ctx.fillRect(180, 80, 5, 5);
ctx.fillRect(160, 180, 5, 5);
ctx.moveTo(62, 112);
ctx.lineTo(182, 82);
ctx.lineTo(162, 182);
// 绘制切线弧
ctx.moveTo(103, 103);
ctx.arcTo(183, 83, 162, 182, 40);
ctx.stroke();

ctx.draw();
```
