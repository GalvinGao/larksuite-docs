---
document_id: '7180270043522121734'
directory_id: '7180165099251023877'
title: CanvasContext.fillRect
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-fillRect
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.fillRect
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-fillRect
---

# CanvasContext.fillRect

## CanvasContext.fillRect(number x, number y, number width, number height)

填充一个矩形，不会被添加到当前路径中。默认的填充色为黑色。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|x|number|-|是|矩形起始点的 x 轴坐标|1.6.0
|y|number|-|是|矩形起始点的 y 轴坐标|1.6.0
|width|number|-|是|矩形的宽度|1.6.0
|height|number|-|是|矩形的高度|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.fillStyle = "green";
ctx.fillRect(10, 10, 150, 75);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4da4122476923231bca68dae3484d06b_SmJ6M1LDSD.png"/>
:::

## Tip
无
