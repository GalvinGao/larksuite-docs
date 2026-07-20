---
document_id: '6965379543684349958'
directory_id: '6907567266541813762'
title: view
full_path: /uYjL24iN/ukTNukTNukTN
breadcrumb:
- Client API
- Gadget Basic Components (Not Recommended)
- View Container
- view
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:53Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukTNukTNukTN
---

# view

视图容器，类似 DIV。

|属性名|类型|默认值|说明|
|----|--|--|--|
|hover-class|String|none|指定按下去的样式类。<br>当 hover-class="none" 时，没有点击态效果。|
|hover-start-time|Number|50|按住后多久出现点击态，单位毫秒。|
|hover-stay-time|Number|400|手指松开后点击态保留时间，单位毫秒。|
|hover-stop-propagation|Boolean|false|指定是否阻止本节点的祖先节点出现点击态。|

## 代码示例

:::html

<md-code page="page/component/pages/view/view">

<md-code-item type="TTML">

```html
<view class="container">

  <view class="page-body">
    <view class="page-section">
      <view class="page-section-title">
        <text>{{direction_row}}</text>
      </view>
    <view>
      <view class="page-section-spacing">
        <view class="flex-wrp" style="flex-direction:row;">
          <view hover-class="hover" class="flex-item demo-text-1"></view>
          <view hover-class="hover" class="flex-item demo-text-2"></view>
          <view hover-class="hover" class="flex-item demo-text-3"></view>
        </view>
      </view>
    </view>
    <view class="page-section">
      <view class="page-section-title">
        <text>{{direction_column}}</text>
      </view>
      <view class="flex-wrp" style="flex-direction:column;">
        <view class="flex-item flex-item-V demo-text-1"></view>
        <view class="flex-item flex-item-V demo-text-2"></view>
        <view class="flex-item flex-item-V demo-text-3"></view>
      </view>
    </view>
  </view>
</view>
```

</md-code-item>

<md-code-item type="JS">

```js
const iText = i18n.view

Page({
  data: {
    ...iText
  }
})
```

</md-code-item>

<md-code-item type="TTSS">

```css
.flex-wrp{
  display:flex;
}
.flex-item{
  width: 200rpx;
  height: 300rpx;
  font-size: 26rpx;
}
.flex-item-V{
  margin: 0 auto;
  width: 300rpx;
  height: 200rpx;
}

.page-section{
  margin-top: 40rpx;
}

.hover{
  background-color: green;
}
```

</md-code-item>

<md-code-item type="JSON">

```json
{
    "navigationBarTitleText": "view"
}

```

</md-code-item>

</md-code>

:::

