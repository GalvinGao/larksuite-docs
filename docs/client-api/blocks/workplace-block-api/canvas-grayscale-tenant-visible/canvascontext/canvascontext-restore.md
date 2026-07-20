---
document_id: '7180269945547898885'
directory_id: '7180165099251023877'
title: CanvasContext.restore
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/restore
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.restore
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/restore
---

# CanvasContext.restore

## CanvasContext.restore()

恢复之前保存的绘图上下文。

## 参数说明

无

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

// save the default fill style
ctx.save();
ctx.setFillStyle("red");
ctx.fillRect(10, 10, 150, 100);

// restore to the previous saved state
ctx.restore();
ctx.fillRect(50, 50, 150, 100);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9596b20ded99971218494bd4377b0eca_GuHDMqG7Vf.png"/>
:::

## Tip
无
