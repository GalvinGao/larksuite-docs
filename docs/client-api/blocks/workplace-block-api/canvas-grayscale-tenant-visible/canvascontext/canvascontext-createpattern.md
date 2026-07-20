---
document_id: '7180269945548668933'
directory_id: '7180165099251023877'
title: CanvasContext.createPattern
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-createPattern
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.createPattern
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-createPattern
---

# CanvasContext.createPattern

## CanvasPattern CanvasContext.createPattern(string image, 'repeat' | 'repeat-x' | 'repeat-y' | 'no-repeat' repetition)

对指定的图像创建模式的方法，可在指定的方向上重复元图像。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|image|string|-|是|图片地址，**目前仅支持网络图片 url**|1.6.0
|repetition|'repeat' \| 'repeat-x' \| 'repeat-y' \| 'no-repeat'|-|是|如何重复图像。`repeat` 表示水平竖直方向都重复，`repeat-x` 表示水平方向都重复，`repeat-y` 表示竖直方向都重复，`no-repeat` 表示不重复|1.6.0

## 返回值

CanvasPattern 对象，无额外属性。

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

const pattern = ctx.createPattern("https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/4788e761425c502c0c1302a95ceb920f.png", "repeat-x");
ctx.fillStyle = pattern;
ctx.fillRect(0, 0, 300, 150);
ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b4c3cd2526137a78804dede0793abd10_rYRKtRoQcJ.png"/>
:::

## Tip
无
