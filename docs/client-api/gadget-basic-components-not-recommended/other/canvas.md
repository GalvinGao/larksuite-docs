---
document_id: '6965379543684202502'
directory_id: '6907567269107335170'
title: canvas
full_path: /uYjL24iN/uczNuczNuczN
breadcrumb:
- Client API
- Gadget Basic Components (Not Recommended)
- Other
- canvas
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:59Z
source_url: https://open.larksuite.com/document/uYjL24iN/uczNuczNuczN
---

# canvas

画布。

|属性名|类型|默认值|说明|
|------|--|-----|--|
|canvas-id|String||canvas 组件的标识，必须设置该属性|

::: note
 `canvas`组件的默认宽度为 300px，高度为 225px
:::

## 代码示例

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
