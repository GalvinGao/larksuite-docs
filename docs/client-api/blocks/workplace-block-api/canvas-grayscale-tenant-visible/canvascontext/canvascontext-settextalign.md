---
document_id: '7180270043523072006'
directory_id: '7180165099251023877'
title: CanvasContext.setTextAlign
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setTextAlign
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.setTextAlign
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setTextAlign
---

# CanvasContext.setTextAlign

## CanvasContext.setTextAlign('left' | 'center' | 'right' align)

设置字体的对齐方式。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|align|'left' \| 'center' \| 'right'|-|是|对齐方式|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

const textAlignList = ["left", "center", "right"];
ctx.strokeStyle = "red";
ctx.moveTo(150, 20);
ctx.lineTo(150, 170);
ctx.stroke();
ctx.setFontSize(15);

for (let i = 0; i < textAlignList.length; ++i) {
  ctx.setTextAlign(textAlignList[i]);
  ctx.fillText(`textAlign = ${textAlignList[i]}`, 150, 40 + 20 * i);
}

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d9099ce2b55b15ec8122bc61bdf3ca60_pacLzg0R8e.png"/>
:::

## Tip
无
