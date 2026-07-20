---
document_id: '7180269945544343558'
directory_id: '7180165099251023877'
title: CanvasContext.measureText
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-measureText
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.measureText
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-measureText
---

# CanvasContext.measureText

## TextMetrics CanvasContext.measureText(string text)

测量文字宽度。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|text|string|-|是|需要测量的文字|1.6.0

## 返回值

TextMetrics 对象，对象的属性如下：

|**属性**|**类型**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|width|number|文本的宽度|1.6.0

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.font = "italic bold 20px cursive";
const metrics = ctx.measureText("Hello Block");
console.log("MeasureText Result:", metrics.width);
```

### **返回值示例**

```json
{
  "width": 98.07992553710938
}
```

## Tip
无
