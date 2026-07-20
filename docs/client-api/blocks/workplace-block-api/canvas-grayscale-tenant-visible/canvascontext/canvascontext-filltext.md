---
document_id: '7180269945547587589'
directory_id: '7180165099251023877'
title: CanvasContext.fillText
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-fillText
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.fillText
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-fillText
---

# CanvasContext.fillText

## CanvasContext.fillText(string text, number x, number y, number maxWidth)

以位置 (x, y) 为左上角填充文本 text。如果提供了最大宽度 maxWidth，文本会进行缩放以适应最大宽度。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|text|string|-|是|在画布上绘制的文本|1.6.0
|x|number|-|是|绘制文本的左上角 x 坐标位置|1.6.0
|y|number|-|是|绘制文本的左上角 y 坐标位置|1.6.0
|maxWidth|number|-|否|绘制的最大宽度|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.setFontSize(20);
ctx.fillText("Hello Block!", 20, 20);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3e0784f004432e08b4c01fc73d397a30_GZOjWjc4IV.png"/>
:::

## Tip
无
