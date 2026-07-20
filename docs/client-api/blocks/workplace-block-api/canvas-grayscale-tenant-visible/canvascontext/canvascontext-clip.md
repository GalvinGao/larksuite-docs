---
document_id: '7180270043521875974'
directory_id: '7180165099251023877'
title: CanvasContext.clip
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-clip
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.clip
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-clip
---

# CanvasContext.clip

## CanvasContext.clip()

将当前创建的路径设置为当前剪切路径，限制后续的绘图范围。可以在使用 CanvasContext.clip 方法前通过使用 CanvasContext.save 方法对当前画布区域进行保存，并在以后的任意时间通过 CanvasContext.restore 方法对其进行恢复。

## 参数说明

无

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.save();
ctx.arc(100, 100, 75, 0, Math.PI * 2, false);
ctx.clip();
ctx.fillRect(0, 0, 100, 100);

ctx.restore();
ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/935bb630a00e6dc892cdd79e0c176f49_XT7IPTpv4X.png"/>
:::

## Tip
无
