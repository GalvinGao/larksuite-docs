---
document_id: '6965379543684005894'
directory_id: '6907567266536390657'
title: progress
full_path: /uYjL24iN/uETNuETNuETN
breadcrumb:
- Client API
- Gadget Basic Components (Not Recommended)
- Basic Content
- progress
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:53Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETNuETNuETN
---

# progress

进度条。

|属性名|类型|默认值|说明|
|-----|---|-----|---|
|percent|Number|0|进度|
|stroke-width|Number|6|进度条的垂直宽度|
|color|String|#F85959|进度条颜色，不建议使用，请使用 active-color 。|
|active-color|String|#F85959|进度条颜色|
|background-color|String|#EBEBEB|未达到进度的颜色|
|active|Boolean|false|进度条动画|
|active-mode|String|backwards|backwards: 动画从头播；<br>forwards：动画从上次结束点接着播。|

## 示例代码

:::html

<md-code page="page/component/pages/progress/progress">

<md-code-item type="TTML">

```html
<view class="container">
  <view class="page-body">
    <view class="page-section page-section-gap">

      <view class="progress-box">
        <progress percent="20" stroke-width="3" />
      </view>

      <view class="progress-box">
        <progress percent="40" activeColor="#1aad19" active stroke-width="4" />
      </view>

      <view class="progress-box">
        <progress percent="60" activeColor="#D8D8D8" active stroke-width="5" />
      </view>

      <view class="progress-box">
        <progress percent="80" activeColor="#10AEFF" active stroke-width="6" />
      </view>
    </view>
  </view>

</view>

```

</md-code-item>


<md-code-item type="TTSS">

```css
progress{
  width: 100%;
}
.progress-box{
  display: flex;
  height: 50rpx;
  margin-bottom: 60rpx;
}
.progress-cancel{
  margin-left: 40rpx;
}
```

</md-code-item>

<md-code-item type="JSON">

```json
{
    "navigationBarTitleText": "progress"
}
```

</md-code-item>

</md-code>

:::
