---
document_id: '7180270043522433030'
directory_id: '7180165099251023877'
title: CanvasContext.draw
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-draw
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.draw
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-draw
---

# CanvasContext.draw

## CanvasContext.draw(boolean reverse, function callback)

将之前在绘图上下文中的指令（路径、变形、样式）画到 Canvas 中。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|reverse|boolean|false|否|本次绘制是否接着上一次绘制。即 reserve 参数为 false，则在本次调用绘制之前会先清空画布（包括重置画布的属性，fillStyle，strokeStyle 等）再继续绘制；若 reserve 参数为 true，则保留当前画布上的内容，将本次的绘制内容覆盖在上面。|1.6.0
|callback|() => void|function(){}|否|绘制完成后执行的回调函数|1.6.0

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

ctx.setFillStyle("red");
ctx.fillRect(10, 10, 150, 100);
ctx.draw();
ctx.fillRect(50, 50, 150, 100);
ctx.draw(true, () => {
  console.log("Last Draw End.");
});
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ebb49575df7164b6b678cf32c2c87ffd_oKsc5iLNHT.png"/>
:::

## Tip
无
