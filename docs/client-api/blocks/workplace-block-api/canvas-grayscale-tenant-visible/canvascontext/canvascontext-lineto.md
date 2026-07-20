---
document_id: '7180270043522465798'
directory_id: '7180165099251023877'
title: CanvasContext.lineTo
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-lineTo
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.lineTo
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-lineTo
---

# CanvasContext.lineTo

## CanvasContext.lineTo(number x, number y)

增加一个新点，然后创建一条从上次指定点到目标点的直线。

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

ctx.moveTo(10, 10);
ctx.rect(10, 10, 100, 50);
ctx.lineTo(110, 60);
ctx.stroke();

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/67cf37dd50e8e88ff677d5805ef782b0_zgeWQzyCZB.png"/>
:::

## Tip

- 需要用 CanvasContext.stroke 方法来在 Canvas 中绘制线条。
