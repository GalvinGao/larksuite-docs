---
document_id: '7180269945547407365'
directory_id: '7180165099251023877'
title: CanvasContext.rotate
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-rotate
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.rotate
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-rotate
---

# CanvasContext.rotate

## CanvasContext.rotate(number angle)

以原点为中心顺时针旋转当前坐标轴。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|angle|number|-|是|旋转弧度|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.strokeRect(100, 10, 150, 100);
ctx.rotate(20 * Math.PI / 180);
ctx.strokeRect(100, 10, 150, 100);
ctx.rotate(20 * Math.PI / 180);
ctx.strokeRect(100, 10, 150, 100);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e357c43fd61c1d1e0700110e90cd75e5_spcWqjKCeo.png"/>
:::

## Tip
无
