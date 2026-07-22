---
document_id: '7073691561008168965'
directory_id: '7073450228347256837'
title: CanvasContext.rect
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-rect
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.rect
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-rect
---

# CanvasContext.rect(number x, number y, number w, number h)

添加矩形到当前路径中

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| x | number | 是 |  | 绘制开始点的 x 坐标 |
| y | number | 是 |  | 绘制开始点的 y 坐标 |
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
const ctx = tt.createCanvasContext(canvasId);

ctx.rect(10, 10, 100, 100);
ctx.fill();

ctx.draw();
```
