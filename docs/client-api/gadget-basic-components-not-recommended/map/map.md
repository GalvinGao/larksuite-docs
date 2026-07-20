---
document_id: '7073692582769246214'
directory_id: '6907567269107957762'
title: map
full_path: /uYjL24iN/uEDM5UjLxATO14SMwkTN
breadcrumb:
- Client API
- Gadget Basic Components (Not Recommended)
- Map
- map
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:56Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEDM5UjLxATO14SMwkTN
---

# map
::: note
Lark3.16开始支持
:::

::: note
**PC端暂不支持**
:::

地图（暂不支持同层渲染）。相关api tt.createMapContext。

| 属性名         | 类型           | 默认值     |必填| 说明         | 最低版本
| --------- | --------------- | -------   | -------   | ----------- | ------|
|`longitude` | `number` |  | 是 |中心经度|3.16
|`latitude` | `number` |  |是|中心纬度|3.16
|`scale` | `number` |  16| 否|缩放级别，取值范围3-20|3.16
|`markers` | `Array<marker>` |  | 否|标记点|3.16
|`show-location` | `boolean` | false |否 |显示带有方向的当前定位点|3.16
|`circles` | `Array<marker>` |  | 否|圆|3.18


## `marker`

元素描述：标记点，用于在地图上显示标记位置

| 属性名         | 类型            | 说明         | 必填|
| --------- | -----------   | ----------- | ---------- | 
|id|number|标记id|否
|latitude|number|纬度|是
|longitude|number|经度|是
|title|String|标记点名|否
|iconPath|String|显示的图标|否
## `circle`

在地图上显示圆

| 属性名         | 类型            | 说明         | 必填|备注
| --------- | -----------   | ----------- | ---------- | -----|
|latitude|number|纬度|是|浮点数，范围 -90 ~ 90
|longitude|number|经度|是|浮点数，范围 -180 ~ 180
|color|String|描边的颜色|否|十六进制
|fillColor|String|填充颜色	|否|十六进制
|radius|number|半径|是
|strokeWidth|number|描边的宽度|否
## 比例尺
| scale | 3 | 4 | 5 | 6 | 7 | 8 |9  | 10 |11|
| ----- | :--: | :--: | :--: | :--:| :--: | :--:| :--: | :--:| :--: 
| 比例 | 1000km | 500km | 200km | 100km | 50km | 30km |20km  | 10km |5km|

| scale  |12 | 13 |14| 15 | 16 | 17 | 18 |19  | 20 |
| ----- | :--: | :--: | :--: | :--:| :--: | :--:| :--: | :--:| :--: 
| 比例 | 2km | 1km | 500m | 200m | 100m | 50m |25m  | 10m |10m|

## 代码示例
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/component/pages/map/map" fontSize="16" style="margin-right: 24px">扫码预览效果</md-preview-app>
  </div>
</div> 
:::



TTML文件：
```html 
<view class="page-body">
  <view class="page-section page-section-gap">
    <map
      id="myMap"
      style="width: 100%; height: 300px;"
      latitude="{{latitude}}"
      scale="{{scale}}"
      longitude="{{longitude}}"
      markers="{{markers}}"
      covers="{{covers}}"
      circles="{{circles}}"
      show-location
    ></map>
   
  </view>
  <view class="btn-area">
    <view >{{show_marks_text}}</button>
    <button bindtap="moveToLocation" class="page-body-button" type="primary">{{move_to_location}}</button>
    <button bindtap="move" class="page-body-button" type="primary">{{move_up}}</button>
    <button bindtap="addZoom" class="page-body-button" type="primary">{{scale_up}}</button>
  </view>
</view> 
``` 
JS文件：
```javascript 
const map = i18n.map

Page({
  data: {
    ...map,
    latitude: 23.099994,
    longitude: 113.324520,
    scale:15,
    markers: [{
      id: 1,
      latitude: 23.099994,
      longitude: 113.324520,
      title: 'T.I.T 创意园',
      iconPath: '/image/location.png'
    },
    {
      latitude: 23.099994,
      longitude: 113.344520,
      title: '111111',
      iconPath: '/image/location.png'
    }, {
      latitude: 23.099994,
      longitude: 113.304520,
      title: '22222',
      iconPath: '/image/location.png'
    }],
    covers: [{
      latitude: 23.499994,
      longitude: 113.344520,
      iconPath: '/image/location.png'
    }, {
      latitude: 23.099994,
      longitude: 113.304520,
      iconPath: '/image/location.png'
    }],
    circles: [{
      latitude: 23.099994,
      longitude: 113.324520,
      radius: 500,
      color: "#313131",
      fillColor: "#121212",
      strokeWidth: 10
    }]
  },
  onReady: function (e) {
    this.mapCtx = tt.createMapContext('myMap',this)
   
  },
  getCenterLocation: function () {
    this.mapCtx.getCenterLocation({
      success: function(res){
        console.log(res.longitude)
        console.log(res.latitude)
      }
    })
  },
  moveToLocation: function () {
      console.log("moveToLocation")
    this.mapCtx.moveToLocation()
  },
  translateMarker: function() {
    this.mapCtx.translateMarker({
      markerId: 1,
      autoRotate: true,
      duration: 1000,
      destination: {
        latitude:23.10229,
        longitude:113.3345211,
      },
      animationEnd() {
        console.log('animation end')
      }
    })
  },
  move: function(){
    console.log("move")
    this.setData({
        latitude:this.data.latitude+1
    })
},
  addZoom: function(){
      console.log("addzoom")
      this.setData({
          scale:this.data.scale+1
      })
      console.log("addzoom")
  },
  includePoints: function() {
    this.mapCtx.includePoints({
      padding: [10],
      points: [{
        latitude:23.10229,
        longitude:113.3345211,
      }, {
        latitude:23.00229,
        longitude:113.3345211,
      }]
    })
  }
})
 
``` 






