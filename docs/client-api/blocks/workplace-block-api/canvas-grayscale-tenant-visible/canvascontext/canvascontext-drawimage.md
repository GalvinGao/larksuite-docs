---
document_id: '7180269945548029957'
directory_id: '7180165099251023877'
title: CanvasContext.drawImage
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-drawImage
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.drawImage
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-drawImage
---

# CanvasContext.drawImage

## CanvasContext.drawImage(string imageResource, number sx, number sy, number sWidth, number sHeight, number dx, number dy, number dWidth, number dHeight)

绘制图像到画布。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|image|string|-|是|所要绘制的图片资源，**目前仅支持网络图片 url**|1.6.0
|sx|number|-|是|需要绘制到画布中的，imageResource 的矩形（裁剪）选择框的左上角 x 轴坐标|1.6.0
|sy|number|-|是|需要绘制到画布中的，imageResource 的矩形（裁剪）选择框的左上角 y 轴坐标|1.6.0
|sWidth|number|-|否|需要绘制到画布中的，imageResource 的矩形（裁剪）选择框的宽度|1.6.0
|sHeight|number|-|否|需要绘制到画布中的，imageResource 的矩形（裁剪）选择框的高度|1.6.0
|dx|number|-|否|imageResource 的左上角在目标画布上 x 轴的位置|1.6.0
|dy|number|-|否|imageResource 的左上角在目标画布上 y 轴的位置|1.6.0
|dWidth|number|-|否|在目标画布上绘制 imageResource 的宽度，允许对绘制的 imageResource 进行缩放|1.6.0
|dHeight|number|-|否|在目标画布上绘制 imageResource 的高度，允许对绘制的 imageResource 进行缩放|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.drawImage(
  "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/4788e761425c502c0c1302a95ceb920f.png",
  0,
  0,
  150,
  100
);
ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d069994c2c9a4ad098e29741b010a8cf_DFiLd4rM92.png"/>
:::

## Tip
无
