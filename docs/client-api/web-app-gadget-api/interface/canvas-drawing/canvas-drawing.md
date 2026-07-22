---
document_id: '6965379541104132101'
directory_id: '6907567266537127937'
title: createCanvasContext
full_path: /uYjL24iN/uMTNy4yM1IjLzUjM
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Canvas Drawing
- Canvas Drawing
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:14Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTNy4yM1IjLzUjM
---

# createCanvasContext(string canvasId)

创建并返回对应 canvasId 的绘图上下文

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/component/pages/canvas/canvas" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| canvasId | string | 是 |  | 需要操纵的 Canvas 组件上的 canvas-id 的值 |


## 输出

返回值：`CanvasContext`，该对象的属性与方法列表参见下表：

:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::
### 绘制

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| [draw](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-draw) | function | 将之前在绘图上下文中的描述（路径、变形、样式）画到 canvas 中 |

### Canvas 状态

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| [save](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-save) | function | 保存绘图上下文 |
| [restore](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-restore) | function | 恢复之前保存的绘图上下文 |

### 线样式

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| [setLineWidth](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineWidth) | function | 设置线条的宽度 |
| lineWidth | number | 线条的宽度。用法同 [CanvasContext.setLineWidth()](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineWidth)<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| [setLineCap](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineCap) | function | 设置线条的端点样式 |
| lineCap | number | 线条的端点样式。用法同 [CanvasContext.setLineCap()](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineCap)<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| [setLineJoin](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineJoin) | function | 设置线条的交点样式 |
| lineJoin | string | 设置线条的交点样式。用法同 [CanvasContext.setLineJoin()](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineJoin)<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| [setMiterLimit](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setMiterLimit) | function | 设置最大斜接长度。斜接长度指的是在两条线交汇处内角和外角之间的距离。当 [CanvasContext.setLineJoin()](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineJoin) 为 `miter` 时才有效。超过最大倾斜长度的，连接处将以 lineJoin 为 bevel 来显示 |
| miterLimit | number | 最大斜接长度。用法同 [CanvasContext.setMiterLimit()](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setMiterLimit)<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| [setLineDash](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineDash) | function | 设置虚线样式 |
| lineDashOffset | number | 虚线偏移量，初始值为0<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |

### 文本样式

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| font | string | 当前字体样式的属性。符合 [CSS font](https://developer.mozilla.org/zh-CN/docs/Web/CSS/font) 语法 的 DOMString 字符串，至少需要提供字体大小和字体族名。默认值为 10px sans-serif。 |
| [setTextAlign](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTextAlign) | function | 设置文字的对齐 |
| textAlign | string | 设置文字的对齐，用法同 [CanvasContext.setTextAlign()](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTextAlign)<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| [setTextBaseline](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTextBaseline) | function | 设置文字的竖直对齐 |
| textBaseline | string | 设置线条的交点样式。用法同 [CanvasContext.setTextBaseline()](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTextBaseline)<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| [setFontSize]() | function | 设置字体的字号 |

### 创建路径

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| [moveTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-moveTo) | function | 把路径移动到画布中的指定点，不创建线条。用 [stroke](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-stroke) 方法来画线条 |
| [closePath](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-closePath) | function | 关闭一个路径。会连接起点和终点。如果关闭路径后没有调用 [fill](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fill) 或者 [stroke](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-stroke) 并开启了新的路径，那之前的路径将不会被渲染 |
| [lineTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-lineTo) | function | 增加一个新点，然后创建一条从上次指定点到目标点的线。用 [stroke](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-stroke) 方法来画线条 |
| [quadraticCurveTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-quadraticCurveTo) | function | 创建二次贝塞尔曲线路径。曲线的起始点为路径中前一个点 |
| [bezierCurveTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-bezierCurveTo) | function | 创建三次方贝塞尔曲线路径。曲线的起始点为路径中前一个点 |
| [arcTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-arcTo) | function | 根据控制点和半径绘制圆弧路径 |
| [arc](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-arc) | function | 创建一条弧线。<br>* 创建一个圆可以指定起始弧度为 0，终止弧度为 2 * Math.PI。<br>* 用 [stroke](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-stroke) 或者 [fill](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fill) 方法来在 canvas 中画弧线。 |
| [rect](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-rect) | function | 创建一个矩形路径。需要用 [fill](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fill) 或者 [stroke](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-stroke) 方法将矩形真正的画到 canvas 中 |

### 转换

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| [scale](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-scale) | function | 在调用后，之后创建的路径其横纵坐标会被缩放。多次调用倍数会相乘 |
| [rotate](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-rotate) | function | 以原点为中心顺时针旋转当前坐标轴。多次调用旋转的角度会叠加。原点可以用 translate 方法修改 |
| [translate](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-translate) | function | 对当前坐标系的原点 (0, 0) 进行变换。默认的坐标系原点为页面左上角 |
| [transform](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-transform) | function | 使用矩阵多次叠加当前变换的方法 |
| [setTransform](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTransform) | function | 使用矩阵重新设置（覆盖）当前变换的方法 |

### 填充和描边样式

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| [setFillStyle](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setFillStyle) | function | 设置填充色 |
| fillStyle | string | 填充颜色。用法同 [CanvasContext.setFillStyle()](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setFillStyle)<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| [setStrokeStyle](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setStrokeStyle) | function | 设置描边颜色 |
| strokeStyle | string | 边框颜色。用法同 [CanvasContext.setStrokeStyle()](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setStrokeStyle)<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| [createLinearGradient](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-createLinearGradient) | function | 创建一个线性的渐变颜色。返回的 `CanvasGradient` 对象需要使用 `CanvasGradient.addColorStop()` 来指定渐变点，至少要两个 |
| createRadialGradient | string | 绘制一个矩形，并用放射状/圆形渐变进行填充<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| [createPattern]() | function | 对指定的图像创建模式的方法，可在指定的方向上重复元图像 |

### 在 Canvas 上绘制矩形

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| [clearRect](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-clearRect) | function | 清除画布上在该矩形区域内的内容 |
| [fillRect](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fillRect) | function | 填充一个矩形。用 [setFillStyle](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setFillStyle) 设置矩形的填充色，如果没设置默认是黑色 |
| [strokeRect](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-strokeRect) | function | 画一个矩形(非填充)。 用 [setStrokeStyle](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setStrokeStyle) 设置矩形线条的颜色，如果没设置默认是黑色 |

### 在 Canvas 上绘制文字

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| [fillText](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fillText) | function | 在画布上绘制被填充的文本 |
| [strokeText](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-strokeText) | function | 给定的 (x, y) 位置绘制文本描边的方法 |
| [measureText](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-measureText) | function | 测量文本尺寸信息。目前仅返回文本宽度。同步接口 |

### 在 Canvas 上绘制路径

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| [beginPath](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-beginPath) | function | 开始创建一个路径。需要调用 [fill](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fill) 或者 [stroke](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-stroke) 才会使用路径进行填充或描边<br>* 同在最开始的时候相当于调用了一次 beginPath。<br>* 同一个路径内的多次 setFillStyle、setStrokeStyle、setLineWidth等设置，以最后一次设置为准。 |
| [fill](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fill) | function | 对当前路径中的内容进行填充。默认的填充色为黑色 |
| [stroke](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-stroke) | function | 画出当前路径的边框。默认颜色色为黑色 |
| [clip](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-clip) | function | 从原始画布中剪切任意形状和尺寸。一旦剪切了某个区域，则所有之后的绘图都会被限制在被剪切的区域内（不能访问画布上的其他区域）。可以在使用 [clip](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-clip) 方法前通过使用 [save](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-save) 方法对当前画布区域进行保存，并在以后的任意时间通过 [restore](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-restore) 方法对其进行恢复 |

### 在 Canvas 上绘制图片

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| [drawImage](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-drawImage) | function | 绘制图像到画布 |

### 合成

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| [setGlobalAlpha](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setGlobalAlpha) | function | 设置全局画笔透明度 |
| globalAlpha | number | 全局画笔透明度。范围 0-1，0 表示完全透明，1 表示完全不透明<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| globalCompositeOperation | string | 在绘制新形状时应用的合成操作的类型。目前安卓版本只适用于 [fill](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fill) 填充块的合成，用于 [stroke](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-stroke) 线段的合成效果都是 source-over。<br>目前支持的操作有<br>* 安卓：xor, source-over, source-atop, destination-out, lighter, overlay, darken, lighten, hard-light<br>* iOS：xor, source-over, source-atop, destination-over, destination-out, lighter, multiply, overlay, darken, lighten, color-dodge, color-burn, hard-light, soft-light, difference, exclusion, saturation, luminosity<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |

### 阴影

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| [setShadow](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setShadow) | function | 设定阴影样式 |
| shadowColor | number | 阴影的颜色<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| shadowOffsetX | number | 阴影相对于形状在水平方向的偏移<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| shadowOffsetY | number | 阴影相对于形状在竖直方向的偏移<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| shadowBlur | number | 阴影的模糊级别<br><md-alert type="tip" icon="none"><br>Lark[V3.45](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |


## 示例代码
:::html

<md-code page="page/component/pages/canvas/canvas">

<md-code-item type="TTML">

```html
<view class="container">
  <view class="page-body">
    <view class="page-body-wrapper">
      <canvas canvas-id="canvas" class="canvas"></canvas>
    </view>
  </view>
</view>
```

</md-code-item>

<md-code-item type="JS">

```js
const canvas = i18n.canvas

Page({
  data :{
    ...canvas
  },
  onShow: function (res) {
    this.position = {
      x: 150,
      y: 150,
      vx: 2,
      vy: 2
    }
    this.interval = setInterval(this.drawBall, 17)
  },
  drawBall: function () {
    var p = this.position
    p.x += p.vx
    p.y += p.vy
    if (p.x >= 300) {
      p.vx = -2
    }
    if (p.x <= 7) {
      p.vx = 2
    }
    if (p.y >= 300) {
      p.vy = -2
    }
    if (p.y <= 7) {
      p.vy = 2
    }

    var context = tt.createCanvasContext('canvas')

    function ball(x, y) {
      context.beginPath(0)
      context.arc(x, y, 5, 0, Math.PI * 2)
      context.setFillStyle('#1aad19')
      context.fill()
      context.stroke()
    }

    ball(p.x, 150)
    ball(150, p.y)
    ball(300 - p.x, 150)
    ball(150, 300 - p.y)
    ball(p.x, p.y)
    ball(300 - p.x, 300 - p.y)
    ball(p.x, 300 - p.y)
    ball(300 - p.x, p.y)

    console.log('will call context.draw');
    context.draw();
  },
  onUnload: function () {
    clearInterval(this.interval)
  }, exportImage: function () {
    tt.canvasToTempFilePath({
      canvasId: 'canvas',
      fileType: "jpg",
      x: 100,
      y: 200,
      width: 100,
      height: 200,
      destWidth: 300,
      destHeight: 400,
      quality: 1,
      success: function (res) {
        console.log(" canvasToTempFilePath success")
        console.log(res.tempFilePath)
        tt.showToast({
          title: "success"
        })
      }, fail: function () {
        console.log(" canvasToTempFilePath fail")
      }

    })
  }
})

```

</md-code-item>

<md-code-item type="TTSS">

```css
.canvas {
  width: 305px;
  height: 305px;
  background-color: #fff;
}
```

</md-code-item>

<md-code-item type="JSON">

```json
{
    "navigationBarTitleText": "canvas"
}
```

</md-code-item>

</md-code>

:::



## 已知问题
* tt Canvas API 的调用请放在 Page 中，否则可能有些问题。
* Canvas的高度、宽度设置不能超过Android、iOS设备的限制，具体限制可参考[相关文档](https://developer.apple.com/library/archive/documentation/DeviceInformation/Reference/iOSDeviceCompatibility/Displays/Displays.html)。


