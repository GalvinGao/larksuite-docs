---
document_id: '7073692582769901574'
directory_id: '7073450228347256837'
title: CanvasContext.setLineWidth
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineWidth
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.setLineWidth
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineWidth
---

# CanvasContext.setLineWidth(number width)

设置线宽

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| width | number | 是 |  | 线宽，单位 px |


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
const lineWidthList = [5, 10, 15, 20];

for (let i = 0; i < lineWidthList.length; ++i) {
  ctx.setLineWidth(lineWidthList[i]);
  ctx.beginPath();
  ctx.moveTo(10, 10 + i * 20);
  ctx.lineTo(150, 10 + i * 20);
  ctx.stroke();
}

ctx.draw();
```
