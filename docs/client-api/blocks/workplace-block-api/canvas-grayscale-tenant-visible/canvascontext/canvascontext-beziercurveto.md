---
document_id: '7180270043522973702'
directory_id: '7180165099251023877'
title: CanvasContext.bezierCurveTo
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/beziercurveto
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.bezierCurveTo
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/beziercurveto
---

# CanvasContext.bezierCurveTo

## CanvasContext.bezierCurveTo(number cp1x, number cp1y, number cp2x, number cp2y, number x, number y)

创建三次贝赛尔曲线路径。该方法需要三个点，第一第二个点是控制点，第三个点是结束点。起始点是当前路径的最后一个点。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|cp1x|number|-|是|控制点 1 的 x 轴坐标|1.6.0
|cp1y|number|-|是|控制点 1 的 y 轴坐标|1.6.0
|cp2x|number|-|是|控制点 2 的 x 轴坐标|1.6.0
|cp2y|number|-|是|控制点 2 的 y 轴坐标|1.6.0
|x|number|-|是|结束点的 x 轴坐标|1.6.0
|y|number|-|是|结束点的 y 轴坐标|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.beginPath();
ctx.moveTo(50, 20);
ctx.bezierCurveTo(230, 30, 150, 60, 50, 100);
ctx.stroke();

ctx.fillStyle = "blue";
// start point
ctx.fillRect(50, 20, 10, 10);
// end point
ctx.fillRect(50, 100, 10, 10);

ctx.fillStyle = "red";
// control point one
ctx.fillRect(230, 30, 10, 10);
// control point two
ctx.fillRect(150, 70, 10, 10);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/14d8f2a493b2833bb2002c08b744c356_Cq2hCwJl0v.png"/>
:::

## Tip
无
