---
document_id: '7180269945547276293'
directory_id: '7180165099251023877'
title: CanvasContext.setGlobalAlpha
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setGlobalAlpha
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.setGlobalAlpha
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-setGlobalAlpha
---

# CanvasContext.setGlobalAlpha

## CanvasContext.setGlobalAlpha(number alpha)

设置全局画笔的透明度。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|alpha|number|-|是|透明度。范围 0 - 1，0 表示完全透明，1 表示完全不透明。|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.setFillStyle("red");
ctx.fillRect(10, 10, 150, 100);
ctx.setGlobalAlpha(0.2);
ctx.setFillStyle("blue");
ctx.fillRect(50, 50, 150, 100);
ctx.setFillStyle("yellow");
ctx.fillRect(100, 100, 150, 100);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/231cb60cc85830108d56b94518b71946_rejx6TP3IQ.png"/>
:::

## Tip
无
