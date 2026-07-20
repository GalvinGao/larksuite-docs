---
document_id: '7180270043522138118'
directory_id: '7180165099251023877'
title: CanvasContext.setFillStyle
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setFillStyle
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.setFillStyle
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setFillStyle
---

# CanvasContext.setFillStyle

## CanvasContext.setFillStyle(string | CanvasPattern | CanvasGradient style)

设置填充的样式。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|style|string \| CanvasPattern \| CanvasGradient|-|是|填充样式，兼容 CSS Color 值、模式对象、渐变对象|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.setFillStyle("red");
ctx.fillRect(10, 10, 150, 75);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9b1a0fae5ccfcdd6a23012150eec8be0_1plMbZwLmt.png"/>
:::

## Tip
无
