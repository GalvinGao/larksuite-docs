---
document_id: '7180270043523055622'
directory_id: '7180165099251023877'
title: CanvasContext.setLineCap
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setLineCap
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.setLineCap
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setLineCap
---

# CanvasContext.setLineCap

## CanvasContext.setLineCap('butt' | 'round' | 'square' lineCap)

设置线条的端点样式。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|lineCap|'butt' \| 'round' \| 'square'|'butt'|否|`butt` 表示线段末端以方形结束。`round` 表示线段末端以圆形结束。`square` 表示线段末端以方形结束，但是增加了一个宽度和线段相同，高度是线段厚度一半的矩形区域。|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

const lineCap = ["butt", "round", "square"];

ctx.strokeStyle = "blue";
ctx.beginPath();
ctx.moveTo(10, 10);
ctx.lineTo(140, 10);
ctx.moveTo(10, 140);
ctx.lineTo(140, 140);
ctx.stroke();

ctx.strokeStyle = "black";
for (let i = 0; i < lineCap.length; ++i) {
  ctx.lineWidth = 15;
  ctx.setLineCap(lineCap[i]);
  ctx.beginPath();
  ctx.moveTo(25 + i * 50, 10);
  ctx.lineTo(25 + i * 50, 140);
  ctx.stroke();
}

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/da06d4c4a5860cff861417dc78a8967a_hVGd0h44ph.png"/>
:::

## Tip
无
