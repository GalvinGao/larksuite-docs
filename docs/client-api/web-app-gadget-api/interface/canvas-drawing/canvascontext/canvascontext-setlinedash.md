---
document_id: '7073692582769115142'
directory_id: '7073450228347256837'
title: CanvasContext.setLineDash
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineDash
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.setLineDash
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineDash
---

# CanvasContext.setLineDash(number[] segments, number offset)

设置间断线

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| segments | number[] | 是 |  | 间断线的分块 |
| offset | number | 否 |  | 间断线起点偏移值 |


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
let height = 15;

const drawDashedLine = (pattern) => {
  ctx.beginPath();
  ctx.setLineDash(pattern);
  ctx.moveTo(0, height);
  ctx.lineTo(300, height);
  ctx.stroke();
  height += 20;
};

drawDashedLine([]);
drawDashedLine([1, 1]);
drawDashedLine([10, 10]);
drawDashedLine([20, 5]);
drawDashedLine([15, 3, 3, 3]);
drawDashedLine([20, 3, 3, 3, 3, 3, 3, 3]);
drawDashedLine([12, 3, 3]);  // Equals [12, 3, 3, 12, 3, 3]

ctx.draw();
```
