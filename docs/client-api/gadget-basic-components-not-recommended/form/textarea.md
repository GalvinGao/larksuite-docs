---
document_id: '6965379541104558085'
directory_id: '6907567266536931329'
title: textarea
full_path: /uYjL24iN/uIzNuIzNuIzN
breadcrumb:
- Client API
- Gadget Basic Components (Not Recommended)
- Form
- textarea
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:56Z
source_url: https://open.larksuite.com/document/uYjL24iN/uIzNuIzNuIzN
---

# textarea

多行输入框。

|属性名|类型|默认值|说明|
|----|----|-----|---|
|value|String||输入框的内容|
|placeholder|String||输入框为空时占位符|
|placeholder-style|String||指定 placeholder 的样式|
|disabled|Boolean|false|是否禁用|
|maxlength|Number|140|最大输入长度，设置为 -1 的时候不限制最大长度|
|focus|Boolean|false|是否获得焦点|
|auto-height|Boolean|false|是否自动增高，设置auto-height时，设置height样式不生效|
|fixed|Boolean|false|是否固定位置，如果设置了`position:fixed`，需要将该属性设置为true。 **PC端不支持**|
|cursor-spacing|Number|0|指定软键盘弹出时，与光标的距离是多少。 **PC端不支持**|
|cursor|Number|-1|指定focus时的光标位置。 **PC端暂不支持**|
|adjust-position|boolean|true|键盘弹起时，是否自动上推页面
|selection-start|Number|-1|指定focus时选中片段的起始位置。 **PC端暂不支持**|
|selection-end|Number|-1|指定focus时选中片段的结束位置。 **PC端暂不支持**|
|bindinput|EventHandler||键盘输入时触发，<br>e.detail={value, cursor}|
|bindfocus|EventHandler||输入框聚焦时触发，<br>event.detail={value,height}|
|bindblur|EventHandler||输入框失去焦点时触发，<br>event.detail={value: value}|
|disable-default-padding|boolean|false|是否去掉 iOS 下的默认内边距(仅iOS有效，支持版本：3.46+)|

::: warnning
1. 请勿在 scroll-view、swiper、picker-view 中使用 textarea 组件
2. css 动画对 textarea 组件无效
3. Android端textarea处于组件层级最上层，在弹出组件时注意组件层级关系


:::
## 代码示例

:::html

<md-code page="page/component/pages/textarea/textarea">

<md-code-item type="TTML">

```html
<view class="container">
  <view class="page-body">
    <view class="page-section">
      <view class="page-section-title">{{height_auto_fix_textarea}}</view>
      <view class="textarea-wrp">
        <textarea bindblur="bindTextAreaBlur" maxlength=-1 auto-height />
      </view>
    </view>

    <view class="page-section">
      <view class="page-section-title">{{auto_focus_textare}}</view>
      <view class="textarea-wrp">
        <textarea auto-focus="true" maxlength=-1 style="height: 3em" />
      </view>
    </view>
  </view>

</view>
```

</md-code-item>

<md-code-item type="JS">

```js
const iTextarea = i18n.textarea

Page({
  data: {
    focus: false,
    ...iTextarea
  },
  bindTextAreaBlur: function(e) {
    console.log(e.detail.value)
  }
})
```

</md-code-item>

<md-code-item type="TTSS">

```css
textarea {
    width: 700rpx;
    padding: 25rpx 0;
}
.textarea-wrp {
    padding: 0 25rpx;
    background-color: #fff;
}
```

</md-code-item>

<md-code-item type="JSON">

```json
{
    "navigationBarTitleText": "textarea"
}
```

</md-code-item>

</md-code>

:::

## 已知问题
- PC端 bindconfirm 在按下回车键时不会触发

