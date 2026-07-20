---
document_id: '7180269945548652549'
directory_id: '7180165099250892805'
title: canvas
full_path: /uAjLw4CM/uYjL24iN/block/component/canvas/canvas
breadcrumb:
- Client API
- Blocks
- Component
- canvas（Grayscale Tenant Visible）
- canvas
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:13Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/component/canvas/canvas
---

# Canvas

画布组件，操控画布的相关 API 请移步 [createCanvasContext](/document/uAjLw4CM/uYjL24iN/block/api/canvas/tt-createcanvascontext)。

## 属性

| **名称**      | **类型**                     | **默认值** | **是否必填** | **说明**     |
| ----------- | --------------------------- | ------- | ------ | ---------- | ---------- |
| canvas-id | string | -      | 否      | canvas 组件的唯一标识符 

## 示例代码

:::html
<md-block>
  <md-block-entity>
{
    "blockID": "mock-blockID", 
	"blockTypeID": "blk_610a0f3659c04004c56b2b90", 
    "sourceMeta": {
        
    }, 
    "sourceData": {
    	"tab": "component",
    	"item": "canvas",
    	"isNew": true
    }
}
</md-block-entity>
    <md-block-appLink>
{
    "openDetail": 1, 
    "title": "canvas", 
    "list_page_url": "https://applink.larksuite.com/client/block/workplace/open?appId=cli_a00834ec56f8d01b%26blockTypeId=blk_610a40455f800004c32b6bb6%26sourceData=%7B%22tab%22%3A%22component%22%2C%22item%22%3A%22getSystemInfo%22%7D", 
    "min_lk_ver": {
        "pc": "5.10.0", 
        "mobile": "5.11.0"
    },
    "blockEntity": {
        "sourceData": {
            "type": "component",
            "item": "canvas",
            "isNew": true
        },
  		"blockID": "mock-block"
    }
}
</md-block-appLink>
<md-block-item type="TTML">
```html
<view scroll-y style="height: 600px; width: 100%;background-color: rgba(31, 35, 41, 0.05);">
  <view class="content{{platform === 'pc' ? '' : ' canvas-m-content'}}">
    <view class="content-title">
      <text
        class="{{platform === 'pc' ? '' : 'm-'}}content-title-font">Canvas</text>
    </view>
    <view class="canvas-desc">
      <text>画布</text>
    </view>
    <view class="{{platform === 'pc' ? '' : ' m-canvas-wrap'}}" style="width:{{platform !== 'pc' ? commonData.width+'px' : '300px'}};height:{{platform !== 'pc' ? commonData.width+'px' : '300px'}}">
      <canvas canvas-id="canvas" width="{{platform === 'pc' ? '300' : commonData.width}}" height="{{platform === 'pc' ? '300' : commonData.width}}"
        class="{{platform === 'pc' ? 'canvas' : 'm-canvas'}}"
        style="width:{{platform !== 'pc' ? commonData.width+'px' : '300px'}};height:{{platform !== 'pc' ? commonData.width+'px' : '300px'}}"
        >
      </canvas>
    </view>
  </view>
</view>
```
</md-block-item>
<md-block-item type="TTSS">
```css
.content {
  padding: 40px;
  overflow-y: scroll;
  display: flex;
  flex-direction: column;
  flex-grow: 1;
}

.m-content {
  padding: 24px 16px 0;
}

.m-content-title-font {
  font-size: 24px;
  font-weight: 600;
  line-height: 32px;
}

.m-sub-title-font {
  font-size: 16px;
}

.sub-title-font {
  font-size: 14px;
  font-family: PingFang SC;
  line-height: 22px;
}

.content-title {
  margin-bottom: 8px;
}

.content-title-font {
  font-size: 30px;
  font-weight: bold;
}

.form-arrow {
  position: relative;
}

.property-container {
  margin-top: 24px; 
  padding-left: 16px; 
  box-sizing: border-box; 
  border-top: 0.5px solid rgba(31, 35, 41, 0.15); 
  border-bottom: 0.5px solid rgba(31, 35, 41, 0.15); 
  display: flex; 
  flex-direction: column;
  width: 100%;
}

.property-container-bg {
  background: white; 
}

.canvas {
  background-color: #fff;
  border-radius: 6px;
  width: 300px;
  height: 300px;
}

.canvas-desc {
  color: #646A73;
  font-size: 16px;
  margin-bottom: 22px;
}
.dark-canvas {
  background: #101010;
  border: 1px solid #fff;
}

.m-canvas-wrap {
    overflow: hidden;
    background: #fff;
    border-radius: 8px
}

.m-canvas {
    width: 330px;
    height: 330px;
    border-radius: 6px;
  }
.canvas-m-content {
    padding: 16px;
}
```
</md-block-item>
<md-block-item type="JS">

```javascript
Block({
  data:{
    platform: 'pc',
    commonData: {
      width: 0
    },
  },
  onLoad(){
    this.preRender()
    tt.hideBlockLoading();
  },
  drawBall: function () {
      var p = this.position
      var { max, star } = this.val
      p.x += p.vx
      p.y += p.vy
      if (p.x >= max) {
        p.vx = -2
      }
      if (p.x <= 7) {
        p.vx = 2
      }
      if (p.y >= max) {
        p.vy = -2
      }
      if (p.y <= 7) {
        p.vy = 2
      }
      var context = tt.createCanvasContext('canvas')
      function ball(x, y) {
        context.beginPath(0)
        context.arc(x, y, 7, 0, Math.PI * 2)
        context.setStrokeStyle('rgba(0,0,0,0)')
        context.setFillStyle('#4E83FD')
        context.fill()
        context.stroke()
      }
      ball(p.x, star)
      ball(star, p.y)
      ball(max - p.x, star)
      ball(star, max - p.y)
      ball(p.x, p.y)
      ball(max - p.x, max - p.y)
      ball(p.x, max - p.y)
      ball(max - p.x, p.y)
      context.draw();
    },

    onDestroy() {
      clearInterval(this.interval)
    },

    preRender() {
      const { commonData } = this.data
      if (this.data.platform !== 'pc') {
        tt.getContainerRect({
          success: (res) => {
            let w = res.width - 32
            let w2 = Math.floor(w / 2)
            this.setData({
              commonData: {
                ...commonData,
                width: w
              }
            },() => {
              this.position = {
                x: w2,
                y: w2,
                vx: 2,
                vy: 2
              }
              this.val = {
                max: w,
                star: w2
              }
              this.interval = setInterval(this.drawBall, 17)
            })
          }
        })
        return
      }
      this.position = {
        x: 150,
        y: 150,
        vx: 2,
        vy: 2
      }
      this.val = {
        max: 300,
        star: 150
      }
      this.interval = setInterval(this.drawBall, 17)
    }
})
```

</md-block-item>
</md-block>
:::

