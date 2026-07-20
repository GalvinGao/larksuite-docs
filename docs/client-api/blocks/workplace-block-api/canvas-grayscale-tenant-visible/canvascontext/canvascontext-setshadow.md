---
document_id: '7180270043522613254'
directory_id: '7180165099251023877'
title: CanvasContext.setShadow
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setShadow
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.setShadow
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setShadow
---

# CanvasContext.setShadow

## CanvasContext.setShadow(number x, number y, number blur, string color)

设置阴影样式。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|x|number|0|否|阴影相对于形状在水平方向的偏移|1.6.0
|y|number|0|否|阴影相对于形状在竖直方向的偏移|1.6.0
|blur|number|0|否|阴影的模糊量，数值越大越模糊。范围为 0 - 100。|1.6.0
|color|string|"black"|否|阴影的颜色|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.setFillStyle("red");
ctx.setShadow(10, 50, 50, "blue");
ctx.fillRect(10, 10, 150, 75);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/81a9257b85c055671ac53d39eca11381_FEcsTOAlcp.png"/>
:::

## Tip
无
