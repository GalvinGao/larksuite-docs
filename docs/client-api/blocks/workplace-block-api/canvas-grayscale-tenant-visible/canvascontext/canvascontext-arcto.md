---
document_id: '7180270043522498566'
directory_id: '7180165099251023877'
title: CanvasContext.arcTo
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/arcto
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.arcTo
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/arcto
---

# CanvasContext.arcTo

## CanvasContext.arcTo(number x1, number y1, number x2, number y2, number radius)

根据控制点和半径绘制圆弧路径，使用当前的描点(**前一个 moveTo 或 lineTo 等函数的结束点**)。根据当前描点与给定的控制点 1 连接的直线，和控制点 1 与控制点 2 连接的直线，作为使用指定半径的圆的切线，画出两条切线之间的弧线路径。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|x1|number|-|是|控制点 1 的 x 轴坐标|1.6.0
|y1|number|-|是|控制点 1 的 y 轴坐标|1.6.0
|x2|number|-|是|控制点 2 的 x 轴坐标|1.6.0
|y2|number|-|是|控制点 2 的 y 轴坐标|1.6.0
|radius|number|-|是|圆弧的半径|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.beginPath();
ctx.fillRect(100, 100, 5, 5);
ctx.fillRect(180, 80, 5, 5);
ctx.fillRect(160, 180, 5, 5);
ctx.moveTo(62, 112);
ctx.lineTo(182, 82);
ctx.lineTo(162, 182);
// Draw tangent arc
ctx.moveTo(103, 103);
ctx.arcTo(183, 83, 162, 182, 40);
ctx.stroke();

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/058353078912dac74895cc79dce7c7f7_TFKXtVKq31.png"/>
:::

## Tip

- 需要用 CanvasContext.stroke 或者 CanvasContext.fill 方法来在 Canvas 中绘制弧线。
