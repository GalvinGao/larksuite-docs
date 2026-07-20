---
document_id: '7180269945544228870'
directory_id: '7180165099251023877'
title: CanvasContext.scale
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-scale
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.scale
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-scale
---

# CanvasContext.scale

## CanvasContext.scale(number x, number y)

缩放坐标。多次调用倍数会相乘。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|x|number|-|是|x 轴方向缩放值|1.6.0
|y|number|-|是|y 轴方向缩放值|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.strokeRect(10, 10, 25, 15);
ctx.scale(2, 2);
ctx.strokeRect(10, 10, 25, 15);
ctx.scale(2, 2);
ctx.strokeRect(10, 10, 25, 15);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ad931fe6763bb4f54e9f3d8768b532cd_2Z7pEZK4ll.png"/>
:::

## Tip
无
