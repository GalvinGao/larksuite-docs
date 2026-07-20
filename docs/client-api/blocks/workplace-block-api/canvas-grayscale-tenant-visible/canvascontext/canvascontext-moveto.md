---
document_id: '7180269945548111877'
directory_id: '7180165099251023877'
title: CanvasContext.moveTo
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-moveTo
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.moveTo
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-moveTo
---

# CanvasContext.moveTo

## CanvasContext.moveTo(number x, number y)

将一个新的子路径的起始点移动到 (x, y) 坐标。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|x|number|-|是|目标位置的 x 轴坐标|1.6.0
|y|number|-|是|目标位置的 y 轴坐标|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.moveTo(10, 50);
ctx.lineTo(100, 50);
ctx.stroke();

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/abb2d01a48de81a704be77771dcd24c3_XWfz8GCI4H.png"/>
:::

## Tip
无
