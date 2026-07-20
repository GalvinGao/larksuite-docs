---
document_id: '6965379541105147909'
directory_id: '6907567266536931329'
title: radio-group
full_path: /uYjL24iN/ukjNukjNukjN
breadcrumb:
- Client API
- Gadget Basic Components (Not Recommended)
- Form
- radio-group
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:56Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjNukjNukjN
---

# radio-group

单项选择器，内部由多个`<radio/>`组成。
|属性名|类型|说明|
|-----|---|---|
|bindchange|EventHandle|`<radio-group/>` 中的选中项发生变化时触发 change 事件，<br>event.detail = {value: 选中项 radio 的 value}|


# radio

单选项目
|属性名|类型|默认值|说明|
|-----|---|-----|---|
|value|String|	|`<radio/>` 标识。当该`<radio/>` 选中时，`<radio-group/>` 的 change 事件<br>会携带`<radio/>`的 value|
|checked|Boolean|false|当前是否选中|
|disabled|Boolean|false|是否禁用|
|color|String|	移动端 #3388FF, PC 端 #3370FF |radio 的颜色，同 css 的 color|

## 示例代码

:::html

<md-code page="page/component/pages/radio/radio">

<md-code-item type="TTML">

```html
<view class="container">
  <view class="page-body">
    <view class="page-section page-section-gap">
      <view class="page-section-title">{{default_style}}</view>
      <label class="radio">
        <radio value="r1" checked="true"/>{{selected}}
      </label>
      <label class="radio">
        <radio value="r2" />{{unselected}}
      </label>
    </view>


    <view class="page-section">
      <view class="page-section-title">{{recommend_display_style}}</view>
      <view class="weui-cells weui-cells_after-title">
        <radio-group bindchange="radioChange">
          <label class="weui-cell weui-check__label" tt:for="{{items}}" tt:key="{{item.value}}">

            <view class="weui-cell__hd">
              <radio value="{{item.value}}"
                checked="{{ item.checked }}"/>
            </view>
            <view class="weui-cell__bd">{{item.name}}</view>
          </label>
        </radio-group>
      </view>
    </view>
  </view>

</view>

```

</md-code-item>

<md-code-item type="JS">

```js
const radio = i18n.radio

Page({
  data: {
    items: [
      {value: 'watermelon', name: 'watermelon'},
      {value: 'apple', name: 'apple', checked: 'true'},
      {value: 'pear', name: 'pear'},
      {value: 'banana', name: 'banana'},
      {value: 'orange', name: 'orange'},
      {value: 'grape', name: 'grape'},
    ],
    ...radio
  },
  radioChange: function(e) {
    console.log('Radio emmit change event，value ：', e.detail.value)

    var items = this.data.items;
    for (var i = 0, len = items.length; i < len; ++i) {
      items[i].checked = items[i].value == e.detail.value
    }

    this.setData({
      items: items
    });
  }
})

```

</md-code-item>

<md-code-item type="TTSS">

```css
.radio {
  margin-right: 20rpx;
}
```

</md-code-item>

<md-code-item type="JSON">

```json
{
    "navigationBarTitleText": "radio"
}
```

</md-code-item>

</md-code>

:::

