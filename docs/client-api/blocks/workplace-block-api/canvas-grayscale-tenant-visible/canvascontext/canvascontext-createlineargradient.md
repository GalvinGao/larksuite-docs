---
document_id: '7180269945548013573'
directory_id: '7180165099251023877'
title: CanvasContext.createLinearGradient
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-createLinearGradient
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.createLinearGradient
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-createLinearGradient
---

# CanvasContext.createLinearGradient

## CanvasGradient CanvasContext.createLinearGradient(number x0, number y0, number x1, number y1)

创建一个线性的渐变颜色。返回的 CanvasGradient 对象需要使用 CanvasGradient.addColorStop 方法来指定渐变点，至少要两个。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|x0|number|-|是|起始点的 x 轴坐标|1.6.0
|y0|number|-|是|起始点的 y 轴坐标|1.6.0
|x1|number|-|是|终止点的 x 轴坐标|1.6.0
|y1|number|-|是|终止点的 y 轴a坐标|1.6.0

## 返回值

CanvasGradient 对象，对象的属性如下：

|**属性**|**类型**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|addColorStop|(number stop, string color) => void|添加颜色的渐变点。小于最小 stop 的部分会按最小 stop 的 color 来渲染，大于最大 stop 的部分会按最大 stop 的 color 来渲染。stop 的范围微 0 - 1。|1.6.0

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

// Create linear gradient
const gradient = ctx.createLinearGradient(0, 0, 200, 0);
gradient.addColorStop(0, "red");
gradient.addColorStop(1, "white");

// Fill with gradient
ctx.setFillStyle(gradient);
ctx.fillRect(10, 10, 150, 80);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e77bc3a4803bbd6e6bca685d2a0f2d0c_o9uBlzYHjj.png"/>
:::

## Tip
无
