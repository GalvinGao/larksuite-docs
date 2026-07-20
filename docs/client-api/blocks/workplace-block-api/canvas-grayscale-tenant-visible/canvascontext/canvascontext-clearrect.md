---
document_id: '7180269945548226565'
directory_id: '7180165099251023877'
title: CanvasContext.clearRect
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/clearrect
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.clearRect
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/clearrect
---

# CanvasContext.clearRect

## CanvasContext.clearRect(number x, number y, number width, number height)

将一个以 (x, y) 为左上角端点，width、height 为宽度和高度的矩形区域像素设置为透明。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|x|number|-|是|矩形左上角横坐标|1.6.0
|y|number|-|是|矩形左上角纵坐标|1.6.0
|width|number|-|是|矩形的宽度|1.6.0
|height|number|-|是|矩形的高度|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.setFillStyle("red");
ctx.fillRect(0, 0, 150, 200);
ctx.setFillStyle("blue");
ctx.fillRect(150, 0, 150, 200);
ctx.clearRect(10, 10, 150, 75);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/de13f5357f031ae465b7e3043510f026_ZwivlgFW9Z.png"/>
:::

## Tip
无
