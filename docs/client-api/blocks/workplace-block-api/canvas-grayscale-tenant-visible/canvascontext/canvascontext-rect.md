---
document_id: '7180270043522875398'
directory_id: '7180165099251023877'
title: CanvasContext.rect
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-rect
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.rect
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-rect
---

# CanvasContext.rect

## CanvasContext.rect(number x, number y, number width, number height)

创建一个以 (x, y) 为左上角，宽为 width，高为 height 的矩形路径。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|x|number|-|是|矩形路径左上角的 x 轴坐标|1.6.0
|y|number|-|是|矩形路径左上角的 y 轴坐标|1.6.0
|width|number|-|是|矩形的宽度|1.6.0
|height|number|-|是|矩形的高度|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.rect(10, 10, 100, 100);
ctx.fill();

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/442afc64d0c222392d5d91c8944b114f_gb3JflJT0S.png"/>
:::

## Tip
无
