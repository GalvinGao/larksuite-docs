---
document_id: '7180269945547948037'
directory_id: '7180165099251023877'
title: CanvasContext.fill
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-fill
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.fill
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-fill
---

# CanvasContext.fill

## CanvasContext.fill()

对当前路径中的内容进行填充。默认的填充色为黑色。

## 参数说明

无

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

// begin path
ctx.rect(10, 10, 100, 30);
ctx.fillStyle = "yellow";
ctx.fill();

ctx.beginPath();
ctx.rect(10, 40, 100, 30);

// only fill this rect, not in current path
ctx.setFillStyle("blue");
ctx.fillRect(10, 70, 100, 30);

ctx.rect(10, 100, 100, 30);

// it will fill current path
ctx.setFillStyle("red");
ctx.fill();
ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fe68c9075f5cbc332b57cda10c843470_E1px4f3to2.png"/>
:::

## Tip

- CanvasContext.fill 填充的的路径是从 CanvasContext.beginPath 开始计算的，但是不会将 CanvasContext.fillRect 包含进去。

**e.g:**

```js
const ctx = tt.createCanvasContext('canvas');

ctx.moveTo(10, 10); 
ctx.lineTo(100, 10);
ctx.lineTo(100, 100);
ctx.closePath();

// 上面 closePath 后没有进行绘制，所以最终只会绘制下面的路径
ctx.beginPath();
ctx.strokeStyle = 'red';
ctx.moveTo(20,20);
ctx.lineTo(20, 100);
ctx.stroke();

ctx.draw();
```
**结果:**

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/30caf64f3ad86f6a64f03c8049825dd2_swR5A3jzFt.png"/>
:::
