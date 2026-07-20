---
document_id: '7180269945544114182'
directory_id: '7180165099251023877'
title: CanvasContext.setLineWidth
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setLineWidth
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.setLineWidth
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setLineWidth
---

# CanvasContext.setLineWidth

## CanvasContext.setLineWidth(number width)

设置线条的宽度。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|width|number|-|是|线条的宽度，单位 px|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

const lineWidthList = [5, 10, 15, 20];

for (let i = 0; i < lineWidthList.length; ++i) {
  ctx.setLineWidth(lineWidthList[i]);
  ctx.beginPath();
  ctx.moveTo(10, 10 + i * 20);
  ctx.lineTo(150, 10 + i * 20);
  ctx.stroke();
}

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/619c5fdec3d37df734784c5e8e9e21e3_ym4Pzb3azu.png"/>
:::

## Tip
无
