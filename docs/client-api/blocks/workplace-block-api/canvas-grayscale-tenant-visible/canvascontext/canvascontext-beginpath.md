---
document_id: '7180269945547997189'
directory_id: '7180165099251023877'
title: CanvasContext.beginPath
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/beginpath
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.beginPath
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/beginpath
---

# CanvasContext.beginPath

## CanvasContext.beginPath()

创建新的路径。

## 参数说明

无

## 返回值

无

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

// First Path
ctx.beginPath();
ctx.strokeStyle = "blue";
ctx.moveTo(20, 20);
ctx.lineTo(200, 20);
ctx.stroke();

// Second path
ctx.beginPath();
ctx.strokeStyle = "green";
ctx.moveTo(20, 20);
ctx.lineTo(120, 120);
ctx.stroke();

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8c306a0a878ceab448174ac95091e859_1LkZ4FLH4c.png"/>
:::

## Tip

- 最开始的时候相当于调用了一次 CanvasContext.beginPath，同一个路径里多次调用样式设置，以最后一次设置为准。
