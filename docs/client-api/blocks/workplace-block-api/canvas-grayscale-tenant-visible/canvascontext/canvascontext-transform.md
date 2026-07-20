---
document_id: '7180269945547964421'
directory_id: '7180165099251023877'
title: CanvasContext.transform
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-transform
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.transform
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-transform
---

# CanvasContext.transform

## CanvasContext.transform(number scaleX, number skewX, number skewY, number scaleY, number translateX, number translateY)

使用矩阵多次叠加当前变换的方法。

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

ctx.transform(1, 1, 0, 1, 0, 0);
ctx.transform(1, 1, 0, 1, 0, 0);
ctx.fillRect(0, 0, 100, 100);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/921ea9187040fc4c693479ff15bfa8ea_7W3Y3DgCsD.png"/>
:::

## Tip
无
