---
document_id: '6965379541104164869'
directory_id: '6907567266536931329'
title: input
full_path: /uYjL24iN/uUjNuUjNuUjN
breadcrumb:
- Client API
- Gadget Basic Components (Not Recommended)
- Form
- input
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:56Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjNuUjNuUjN
---

# input

输入框。

|属性名|类型|默认值|说明|
|-----|---|-----|---|
|value|String||输入框的默认值|
|type|String|text|input 的类型，详情见后面的说明|
|password|Boolean|false|是否为密码输入|
|placeholder|String||占位字符|
|placeholder-style|String||占位符的样式|
|disabled|Boolean|false|是否禁用|
|maxlength|Number|140|最大输入长度|
|focus|Boolean|false|获取焦点|
|bindinput|EventHandler||键盘输入时触发，e.detail={value, cursor}；<br>处理函数可以直接 `return` 一个字符串，将替换 input 框的内容，该特性目前 PC 端不支持。|
|bindfocus|EventHandler||输入框聚焦时触发，<br>event.detail={value,height}，height 为键盘高度|
|bindblur|EventHandler||输入框失去焦点时触发，<br>event.detail={value: value}|
|bindconfirm|EventHandler||用户点击键盘的完成按钮时触发，<br>event.detail={value: value}|

type的取值范围：

|值|说明|
|--|--|
|text|文本输入键盘|
|number|数字输入键盘|
|idcard|身份证输入键盘，支持输入[0-9]以及‘X’ **(Lark[V5.2.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上支持输入‘X’，PC端不支持身份证输入键盘)**|
|digit|带小数点的数字键盘，支持输入[0-9]，‘-’，‘.’以及‘,’ **(Lark[V5.2.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上支持输入‘-‘，’,’)**|

::: warnning
避免过于频繁地在 bindinput 回调里执行 `setData({value: ...})`，如果想在键盘输入时改变 input 框的值，可以直接在 bindinput 回调里 `return` 一个字符串
:::

示例代码：

```html
<view class="page-cells page-cells_after-title">
    <view class="page-cell page-cell_input">
        <input class="page-input"
            placeholder="这个只有在按钮点击的时候才聚焦" focus="{{focus}}" />
    </view>
</view>
<view class="btn-area">
  <button bindtap="bindButtonTap">使得输入框获取焦点</button>
</view>

<view class="page-cells page-cells_after-title">
<view class="page-cell page-cell_input">
    <input class="page-input"
        placeholder="focus详情"
        bindfocus="onfocus" bindconfirm="onconfirm" />
</view>
</view>

<view class="btn-area">{{focusDetail}}</view>

<view class="page-cells page-cells_after-title">
<view class="page-cell page-cell_input">
    <input class="page-input" placeholder="blur测试" bindblur="onblur" />
</view>
</view>
```

```js
Page({
  data: {
    focus: false,
  },
  bindButtonTap: function (e) {
    this.setData({
      focus: true,
    })
  },
  onfocus: function (e) {
    console.log(e)
    this.setData({focusDetail: JSON.stringify(e.detail)})
  },
  onblur: function (e) {
    tt.showToast({title: 'blur'})
  },
  onconfirm: function (e) {
    tt.showToast({title: 'confirm'})
  }
})
```
<br>
![图片名称](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/1687b85fe1a110015bf890d9ce9ba766.png)
<br>
