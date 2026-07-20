---
document_id: '7180270043523137542'
directory_id: '7180165099251023877'
title: CanvasContext.quadraticCurveTo
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-quadraticCurveTo
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.quadraticCurveTo
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-quadraticCurveTo
---

# CanvasContext.quadraticCurveTo

## CanvasContext.quadraticCurveTo(number cpx, number cpy, number x, number y)

创建二次贝赛尔曲线路径。该方法需要两个点，第一个点是控制点，第二个点是结束点。起始点是当前路径的最后一个点。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|cpx|number|-|是|控制点的 x 轴坐标|1.6.0
|cpy|number|-|是|控制点的 y 轴坐标|1.6.0
|x|number|-|是|结束点的 x 轴坐标|1.6.0
|y|number|-|是|结束点的 y 轴坐标|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

// Quadratic Bézier curve
ctx.beginPath();
ctx.moveTo(50, 20);
ctx.quadraticCurveTo(230, 30, 50, 100);
ctx.stroke();

// Start and end points
ctx.fillStyle = "blue";
ctx.beginPath();
ctx.arc(50, 20, 5, 0, 2 * Math.PI);   // Start point
ctx.fill();
ctx.beginPath();
ctx.arc(50, 100, 5, 0, 2 * Math.PI);  // End point
ctx.fill();

// Control point
ctx.fillStyle = "red";
ctx.beginPath();
ctx.arc(230, 30, 5, 0, 2 * Math.PI);
ctx.fill();

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c1ca6c1bcd561a5748f17311b1751a92_Eo5z8Xv82d.png"/>
:::

## Tip
无
