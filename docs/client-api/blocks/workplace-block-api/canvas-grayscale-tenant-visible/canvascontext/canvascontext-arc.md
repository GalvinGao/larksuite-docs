---
document_id: '7180270043523203078'
directory_id: '7180165099251023877'
title: CanvasContext.arc
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/arc
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.arc
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/arc
---

# CanvasContext.arc

## CanvasContext.arc(number x, number y, number radius, number startAngle, number endAngle, boolean counterclockwise)

创建一条弧线。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|x|number|-|是|圆弧中心的 x 坐标|1.6.0
|y|number|-|是|圆弧中心的 y 坐标|1.6.0
|radius|number|-|是|圆弧的半径|1.6.0
|startAngle|number|-|是|起始弧度，x 轴方向开始计算，单位弧度|1.6.0
|endAngle|number|-|是|终止弧度，单位弧度|1.6.0
|counterclockwise|boolean|false|否|弧度的方向是否是逆时针|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.arc(100, 100, 60, 0, 1.5 * Math.PI);
ctx.fill();
ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9b3e1490bd12875d903467089758cd68_QDdNug4suf.png"/>
:::

## Tip

- 需要用 CanvasContext.stroke 或者 CanvasContext.fill 方法来在 Canvas 中绘制弧线。
