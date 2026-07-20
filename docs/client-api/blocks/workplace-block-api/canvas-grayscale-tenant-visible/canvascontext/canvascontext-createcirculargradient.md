---
document_id: '7180270043522859014'
directory_id: '7180165099251023877'
title: CanvasContext.createCircularGradient
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-createCircularGradient
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- CanvasContext
- CanvasContext.createCircularGradient
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/canvascontext/canvascontext-createCircularGradient
---

# CanvasContext.createCircularGradient

## CanvasGradient CanvasContext.createCircularGradient(number x, number y, number radius)

创建一个圆形的渐变颜色。起点在圆心，终点在圆环。返回的 CanvasGradient 对象需要使用 CanvasGradient.addColorStop 方法来指定渐变点，至少要两个。

## 参数说明

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|x|number|-|是|圆心的 x 轴坐标|1.6.0
|y|number|-|是|圆心的 y 轴坐标|1.6.0
|radius|number|-|是|圆的半径|1.6.0

## 返回值

CanvasGradient 对象，对象的属性如下：

|**属性**|**类型**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
|addColorStop|(number stop, string color) => void|添加颜色的渐变点。小于最小 stop 的部分会按最小 stop 的 color 来渲染，大于最大 stop 的部分会按最大 stop 的 color 来渲染。stop 的范围微 0 - 1。|1.6.0

## 示例

### 示例代码

```javascript
const ctx = tt.createCanvasContext("canvas");

const gradient = ctx.createCircularGradient(100, 100, 100);
gradient.addColorStop(0, "white");
gradient.addColorStop(1, "green");
ctx.fillStyle = gradient;
ctx.fillRect(0, 0, 300, 200);

ctx.draw();
```

### 示例效果

:::html
<img style="width: 300px; height: 200px;" src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3c6bb4859d9a37aa7a8faec078433409_lH95BCtBma.png"/>
:::

## Tip
无
