---
document_id: '7073691561008087045'
directory_id: '7073450228347256837'
title: CanvasContext.setMiterLimit
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setMiterLimit
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.setMiterLimit
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setMiterLimit
---

# CanvasContext.setMiterLimit(number limit)

设置线连接点渲染的斜面倾斜程度

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| limit | number | 是 |  | 斜面倾斜程度 |


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
const miterLimitList = [1, 2, 3, 4];

for (let i = 0; i < miterLimitList.length; ++i) {
  ctx.beginPath();
  ctx.setLineWidth(10);
  ctx.lineJoin = "miter";
  ctx.setMiterLimit(miterLimitList[i]);
  ctx.moveTo(10 + 40 * i, 10);
  ctx.lineTo(100 + 40 * i, 50);
  ctx.lineTo(10 + 40 * i, 90);
  ctx.stroke();
}

ctx.draw();
```
