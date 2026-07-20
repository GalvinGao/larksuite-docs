---
document_id: '7180269945547194373'
directory_id: '7180165099251023877'
title: CanvasContext.setStrokeStyle
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setStrokeStyle
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.setStrokeStyle
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setStrokeStyle
---

# CanvasContext.setStrokeStyle

## CanvasContext.setStrokeStyle(string | CanvasGradient style)

设置描边的颜色。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|style|string \| CanvasGradient|-|是|描边的颜色，兼容 CSS Color 值、渐变对象|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.setStrokeStyle("red");
ctx.strokeRect(10, 10, 150, 75);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/917c5a936c6b35b06592a25eb0ecfd3e_q0NIW2swTa.png"/>
:::

## Tip
无
