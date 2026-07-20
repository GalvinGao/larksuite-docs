---
document_id: '7180269945548128261'
directory_id: '7180165099251023877'
title: CanvasContext.setLineJoin
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setLineJoin
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.setLineJoin
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setLineJoin
---

# CanvasContext.setLineJoin

## CanvasContext.setLineJoin('round' | 'bevel' | 'miter' join)

设置 2 个长度不为 0 的相连部分（线段，圆弧，曲线）如何连接在一起的属性。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|join|'round' \| 'bevel' \| 'miter'|'miter'|否|`round` 表示圆角。`bevel` 表示斜角。`miter` 表示尖角。|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

const lineJoin = ["round", "bevel", "miter"];
ctx.lineWidth = 10;

for (let i = 0; i < lineJoin.length; ++i) {
  ctx.setLineJoin(lineJoin[i]);
  ctx.beginPath();
  ctx.moveTo(10, 10 + i * 40);
  ctx.lineTo(50, 50 + i * 40);
  ctx.lineTo(90, 10 + i * 40);
  ctx.lineTo(130, 50 + i * 40);
  ctx.lineTo(170, 10 + i * 40);
  ctx.stroke();
}

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/494daf8193405705a128050a931c1700_ulHiTAZ1Ry.png"/>
:::

## Tip
无
