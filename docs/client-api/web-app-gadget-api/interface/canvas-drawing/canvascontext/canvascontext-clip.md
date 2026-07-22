---
document_id: '7073692582769590278'
directory_id: '7073450228347256837'
title: CanvasContext.clip
full_path: /uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-clip
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- CanvasContext
- CanvasContext.clip
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-clip
---

# CanvasContext.clip()

剪切当前路径，限制后续的渲染范围

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

无

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
ctx.save();
ctx.arc(100, 100, 75, 0, Math.PI * 2, false);
ctx.clip();
ctx.fillRect(0, 0, 100, 100);

ctx.restore();
ctx.draw();
```
