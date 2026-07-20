---
document_id: '7180270043522285574'
directory_id: '7180165099251023877'
title: CanvasContext.setTextBaseline
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setTextBaseline
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.setTextBaseline
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setTextBaseline
---

# CanvasContext.setTextBaseline

## CanvasContext.setTextBaseline('top' | 'middle' | 'bottom' | 'normal' textBaseline)

设置文字垂直方向的对齐方式。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|textBaseline|'top' \| 'middle' \| 'bottom' \| 'normal"'|-|是|垂直方向的对齐方式|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

const textBaselineList = ["top", "middle", "bottom", "normal"];

ctx.strokeStyle = "red";
ctx.moveTo(5, 75);
ctx.lineTo(295, 75);
ctx.stroke();
ctx.font = "20px sans-serif";

for (let i = 0; i < textBaselineList.length; ++i) {
  ctx.setTextBaseline(textBaselineList[i]);
  ctx.fillText(textBaselineList[i], 5 + 70 * i, 75);
}

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3c56bdd03e72e748c5f7e78ab0d75bcc_IzQZhwYej6.png"/>
:::

## Tip
无
