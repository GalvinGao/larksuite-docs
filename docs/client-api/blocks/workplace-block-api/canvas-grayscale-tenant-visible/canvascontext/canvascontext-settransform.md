---
document_id: '7180270043522056198'
directory_id: '7180165099251023877'
title: CanvasContext.setTransform
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setTransform
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.setTransform
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setTransform
---

# CanvasContext.setTransform

## CanvasContext.setTransform(number scaleX, number skewX, number skewY, number scaleY, number translateX, number translateY)

设置坐标转换矩阵。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|scaleX|number|-|是|水平缩放|1.6.0
|skewX|number|-|是|水平倾斜|1.6.0
|skewY|number|-|是|垂直倾斜|1.6.0
|scaleY|number|-|是|垂直缩放|1.6.0
|translateX|number|-|是|水平移动|1.6.0
|translateY|number|-|是|垂直移动|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.setTransform(1, 1, 0, 1, 0, 0);
ctx.fillRect(0, 0, 100, 100);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/80d7bd118f1bad6bd35147f37addb406_DKSZG3mfyC.png"/>
:::

## Tip
无
