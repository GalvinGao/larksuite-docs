---
document_id: '7180269945548537861'
directory_id: '7180165099251023877'
title: CanvasContext.setLineDash
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setLineDash
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.setLineDash
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setLineDash
---

# CanvasContext.setLineDash

## CanvasContext.setLineDash(number[] segments, number offset)

设置线条虚线样式的间距和长度。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|segments|number[]|-|是|一组值来指定描述模式的线和间隙的交替长度。 如果数组元素的数量是奇数，数组的元素会被复制并重复。例如，`[5, 15, 25]` 会变成 `[5, 15, 25, 5, 15, 25]`。|1.6.0
|offset|number|-|是|虚线偏移量|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

let height = 15;

const drawDashedLine = (pattern) => {
  ctx.beginPath();
  ctx.setLineDash(pattern);
  ctx.moveTo(0, height);
  ctx.lineTo(300, height);
  ctx.stroke();
  height += 20;
};

drawDashedLine([]);
drawDashedLine([1, 1]);
drawDashedLine([10, 10]);
drawDashedLine([20, 5]);
drawDashedLine([15, 3, 3, 3]);
drawDashedLine([20, 3, 3, 3, 3, 3, 3, 3]);
drawDashedLine([12, 3, 3]);  // Equals [12, 3, 3, 12, 3, 3]

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ac7f9929def1b505b2aec10d9573fda8_oTb07b8q7H.png"/>
:::

## Tip
无
