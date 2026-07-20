---
document_id: '7180269945544409094'
directory_id: '7180165099251023877'
title: CanvasContext.setMiterLimit
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setMiterLimit
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.setMiterLimit
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setMiterLimit
---

# CanvasContext.setMiterLimit

## CanvasContext.setMiterLimit(number limit)

设置最大斜接长度。斜接长度指的是在两条线交汇处内角和外角之间的距离。当 lineJoin 为 miter 时才有效。超过最大倾斜长度的，连接处将以 lineJoin 为 bevel 来显示。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|limit|number|-|是|最大斜接长度|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

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

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e1f7146aab646a3ae70e0906c9266fe7_BUjXT6ukFX.png"/>
:::

## Tip
无
