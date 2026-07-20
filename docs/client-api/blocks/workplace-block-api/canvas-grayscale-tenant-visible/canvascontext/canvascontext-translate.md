---
document_id: '7180270043522760710'
directory_id: '7180165099251023877'
title: CanvasContext.translate
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-translate
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.translate
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-translate
---

# CanvasContext.translate

## CanvasContext.translate(number x, number y)

对当前坐标系的原点 (0, 0) 进行变换。默认的坐标系原点为页面左上角。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|x|number|-|是|水平坐标平移量|1.6.0
|y|number|-|是|竖直坐标平移量|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.strokeRect(10, 10, 150, 100);
ctx.translate(20, 20);
ctx.strokeRect(10, 10, 150, 100);
ctx.translate(20, 20);
ctx.strokeRect(10, 10, 150, 100);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2e25d9a1b6319963296958d3f7193aa8_2AbsCiIFjk.png"/>
:::

## Tip
无
