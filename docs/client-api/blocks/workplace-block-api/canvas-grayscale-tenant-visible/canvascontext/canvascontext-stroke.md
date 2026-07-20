---
document_id: '7180269945548341253'
directory_id: '7180165099251023877'
title: CanvasContext.stroke
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-stroke
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.stroke
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-stroke
---

# CanvasContext.stroke

## CanvasContext.stroke()

绘制出当前路径的边框。默认颜色为黑色。

## 参数说明

无

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.moveTo(10, 10);
ctx.lineTo(100, 10);
ctx.lineTo(100, 100);
ctx.stroke();

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/200670706eb3c79c7f8716aca15f26ca_LXyQdcGC5n.png"/>
:::

## Tip
无
