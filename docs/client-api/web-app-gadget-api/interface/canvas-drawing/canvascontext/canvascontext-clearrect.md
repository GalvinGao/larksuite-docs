---
document_id: '7073692582769541126'
directory_id: '7073450228347256837'
title: CanvasContext.clearRect
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-clearRect
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.clearRect
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-clearRect
---

# CanvasContext.clearRect(number x, number y, number w, number h)

清空画布矩形区域

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| x | number | 是 |  | 开始点的 x 坐标 |
| y | number | 是 |  | 开始点的 y 坐标 |
| w | number | 是 |  | 矩形宽度 |
| h | number | 是 |  | 矩形高度 |


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
ctx.setFillStyle("red");
ctx.fillRect(0, 0, 150, 200);
ctx.setFillStyle("blue");
ctx.fillRect(150, 0, 150, 200);
ctx.clearRect(10, 10, 150, 75);
```
