---
document_id: '6965379543683153926'
directory_id: '6907567266536931329'
title: slider
full_path: /uYjL24iN/uAzNuAzNuAzN
breadcrumb:
- Client API
- Gadget Basic Components (Not Recommended)
- Form
- slider
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:56Z
source_url: https://open.larksuite.com/document/uYjL24iN/uAzNuAzNuAzN
---

# slider

滑动选择器。

|属性名|类型|默认值|说明|
|-----|---|-----|---|
|min|Number|0|最小值|
|max|Number|100|最大值|
|step|Number|1|步长，取值必须大于 0，并且可被(max-min)整除|
|disabled|Boolean|false|是否禁用|
|value|Number|0|当前取值|
|color|Color|#e9e9e9|背景条的颜色（请使用 background-color）	|
|selected-color|Color|#1aad19|已选择的颜色（请使用 active-color）|
|active-color|Color|#1aad19|已选择的颜色|
|background-color|Color|#e9e9e9|背景条的颜色|
|block-size|Number|28|滑块的大小，取值范围为 12 - 28|
|block-color|Color|#ffffff|滑块的颜色|
|show-value|Boolean|false|是否显示当前 value|
|bindchange|EventHandle||完成一次拖动后触发的事件，event.detail = {value: value}	|
|bindchanging|EventHandle||拖动过程中触发的事件，event.detail = {value: value}|

## 示例代码

:::html

<md-code page="page/component/pages/slider/slider">

<md-code-item type="TTML">

```html
<view class="container">
  <view class="page-body">
    <view class="page-section page-section-gap">
      <view class="page-section-title">{{set_step}}</view>
      <view class="body-view">
        <slider value="60" bindchange="slider2change" step="5"/>
      </view>
    </view>

    <view class="page-section page-section-gap">
      <view class="page-section-title">{{show_current_value}}</view>
      <view class="body-view">
        <slider value="50" bindchange="slider3change" show-value/>
      </view>
    </view>

    <view class="page-section page-section-gap">
      <view class="page-section-title">{{set_range}}</view>
      <view class="body-view">
        <slider value="100" bindchange="slider4change" min="50" max="200" show-value/>
      </view>
    </view>
  </view>

  <template is="foot" />
</view>

```

</md-code-item>

<md-code-item type="JS">

```js
const iSlider = i18n.slider

var pageData = {
  data: {
    ...iSlider
  }
}
for (var i = 1; i < 5; ++i) {
  (function (index) {
    pageData['slider' + index + 'change'] = function(e) {
      console.log('slider' + index + 'change event，value : ', e.detail.value)
    }
  })(i)
}
Page(pageData)
```

</md-code-item>


<md-code-item type="JSON">

```json
{
    "navigationBarTitleText": "slider"
}
```

</md-code-item>

</md-code>

:::
