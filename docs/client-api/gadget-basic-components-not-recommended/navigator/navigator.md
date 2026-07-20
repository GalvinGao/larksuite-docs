---
document_id: '6965379543683842054'
directory_id: '6907567269107941378'
title: navigator
full_path: /uYjL24iN/uMzNuMzNuMzN
breadcrumb:
- Client API
- Gadget Basic Components (Not Recommended)
- Navigator
- navigator
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:56Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMzNuMzNuMzN
---

# navigator


页面链接。

|属性名|类型|默认值|说明|
|----|--|---|--|
|url|String||跳转地址|
|delta|Number||当 open-type 为 'navigateBack' 时有效，表示回退的层数|
|open-type|String|navigate|跳转方式，详情参见后面的说明|
|hover-class|String|navigator-hover|点击时的样式类|
|hover-start-time|Number|50|按住多长时间后出现点击状态，单位毫秒|
|hover-stay-time|Number|400|手指松开后点击状态保留时间，单位毫秒|
|hover-stop-propagation|Boolean|false|指定是否阻止本节点的祖先节点出现点击态|

::: note
navigator-hover 默认为 `{background-color: rgba(0, 0, 0, 0.1); opacity: 0.7;}`, `<navigator/>` 的子节点背景色应为透明色
:::

open-type取值范围：

|值|说明|
|--|----|
|navigate|对应 `tt.navigateTo`|
|redirect|对应 `tt.redirectTo`|
|switchTab|对应 `tt.switchTab`|
|navigateBack|对应 `tt.navigateBack`|

## 代码示例

:::html

<md-code page="page/component/pages/navigator/navigator">

<md-code-item type="TTML">

```html
<view class="container">
  <view class="page-body">
    <view class="btn-area">
      <navigator url="navigate?title=navigate" hover-class="navigator-hover">
        <button type="default">{{open_in_new_page}}</button>
      </navigator>
      <navigator url="redirect?title=redirect" redirect hover-class="other-navigator-hover">
        <button type="default">{{open_in_current_page}}</button>
      </navigator>
    </view>
  </view>

</view>
```

</md-code-item>

<md-code-item type="JS">

```js
const navigator = i18n.navigator
Page({
    data: {
        ...navigator
      },
})
```

</md-code-item>

<md-code-item type="TTSS">

```css
.navigator-hover button{
  background-color: #DEDEDE;
}
.other-navigator-hover button{
  background-color: #DEDEDE;
}
```

</md-code-item>

<md-code-item type="JSON">

```json
{
    "navigationBarTitleText": "navigator"
}
```

</md-code-item>

</md-code>

:::
