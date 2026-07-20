---
document_id: '6965379567070527493'
directory_id: '6907567266540404738'
title: 小程序页面
full_path: /uYjL24iN/uQDNuQDNuQDN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- Framework Interface
- Start page
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:38Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQDNuQDNuQDN
---

# 小程序页面

## Page(Object params)
是进入某个页面的时候会执行的页面入口函数，`params` 是一个 Object 类型的参数，定义了页面初始数据，生命周期钩子函数，事件处理函数等。


## 参数说明

|属性|类型|描述|
|-----|---|-----|
|data|Object|页面的初始数据|
|onLoad|function|生命周期函数--监听页面加载|
|onReady|function|生命周期函数--监听页面初次渲染完成|
|onShow|function|生命周期函数--监听页面显示|
|onHide|function|生命周期函数--监听页面隐藏|
|onUnload|function|生命周期函数--监听页面卸载|
|onPullDownRefresh|function|页面相关事件处理函数--监听用户下拉动作|
|onReachBottom|function|页面上拉触底事件的处理函数|
|onPageScroll|function|页面滚动触发事件的处理函数|
|onTabItemTap|function|当前是 tab 页时，点击 tab 时触发|
|onTabbarDoubleTap|function|当前是 tab 页时，双击 tab 时触发|
|onShareAppMessage|function|当点击页面内转发按钮或者右上角菜单里的分享|
|onSelectionChange|function|当页面组件触发选中事件的时候，比如 text。`Lark ≥ 4.2`|
|其他|any|开发者可以添加任意的函数或数据到 object 参数中，在页面的函数中用 this 可以访问|
<br>


# 注册页面
对于小程序中的每个页面，都需要在页面对应的 js 文件中进行注册，指定页面的初始数据、生命周期回调、事件处理函数等。
## 使用 Page 构造器注册页面
使用 Page() 进行构造。

**代码示例：**

```javascript
// index.js
Page({
  data: {
    text: "This is page data."
  },
  onLoad: function(options) {
    // 页面创建时执行
  },
  onReady: function() {
    // 页面初次渲染完毕时执行
  },
  onShow: function() {
    // 页面出现在前台时执行
  },
  onHide: function() {
    // 页面从前台变为后台时执行
  },
  onUnload: function() {
    // 页面卸载时执行
  },
  onPullDownRefresh: function() {
    // 触发下拉刷新时执行
  },
  onReachBottom: function() {
    // 页面触底时执行
  },
  onShareAppMessage: function () {
    // 页面被用户分享时执行
   return {
      title: 'share title',
      imageUrl: '',
      path: ''
    }
  },
  onPageScroll: function() {
    // 页面滚动时执行
  },
  onTabItemTap(item) {
    // 当前是 tab 页时，点击 tab 时执行
    console.log(item.index)
    console.log(item.pagePath)
    console.log(item.text)
  },
  onTabbarDoubleTap(item) {
    // 当前是 tab 页时，双击 tab 时执行
    console.log(item.index)
    console.log(item.pagePath)
    console.log(item.text)
  }
  // 事件响应函数
  viewTap: function() {
    this.setData({
      text: 'Set some data for updating view.'
    }, function() {
      // setData 的回调
    })
  },
  customData: {
    foo: 'bar'
  }
})
```
## data
`data` 数据是页面第一次渲染使用的**初始数据**。页面加载时，data 将会以 JSON 的形式由逻辑层传至渲染层，所以其数据必须是可以转成 JSON 的格式：字符串、数字、布尔值、对象、数组。
渲染层可以通过 TTML 对数据进行绑定。

<br>
**示例代码**
```javascript
<!-- index.ttml -->
<view>{{text}}</view>
<view>{{array[0].msg}}</view>
```
```javascript
// index.js
Page({ 
  data: { 
    text: 'init data', 
    array: [{msg: '1'}, {msg: '2'}] 
  } 
})
```

## 页面事件处理
除了初始化数据和生命周期函数，Page 中还可以定义一些特殊的函数：事件处理函数。在渲染层的组件中可以加入事件绑定。当事件被触发时，就会执行 Page 中定义的事件处理函数。
<br>
**示例代码**
```javascript
<!-- index.ttml -->
<view bindtap="viewTap"> click me </view>
```
```javascript
// index.js
Page({ 
  viewTap: function() { 
    console.log('view tap') 
  } 
})
```


## 更新页面渲染
**setData(Object data, Function callback)** <br>
`setData` 用于将数据从逻辑层发送到视图层（异步），同时改变对应的 `this.data` 的值（同步）

### 参数
|字段|类型|必填|描述|
|-----|---|-----|-----|
|data|Object|是|这次要改变的数据|
|callback|function|否|回调函数|

`data` 以 `key:value` 的形式表示将 `this.data` 中的 key 对应的值改变成 value。 `callback` 是一个回调函数，在这次 `setData` 对界面渲染完成后调用。

**示例代码**

```html
<!--index.ttml-->
<view>{{text}}</view>
<button bindtap="changeText"> Change normal data </button>
<view>{{num}}</view>
<button bindtap="changeNum"> Change normal num </button>
<view>{{array[0].text}}</view>
<button bindtap="changeItemInArray"> Change Array data </button>
<view>{{object.text}}</view>
<button bindtap="changeItemInObject"> Change Object data </button>
<view>{{newField.text}}</view>
<button bindtap="addNewField"> Add new data </button>
```

```js
//index.js
Page({
  data: {
    text: 'init data',
    num: 0,
    array: [{text: 'init data'}],
    object: {
      text: 'init data'
    }
  },
  changeText: function() {
    // this.data.text = 'changed data'  // 这样无法更新UI
    this.setData({
      text: 'changed data'
    })
  },
  changeNum: function() {
    this.data.num = 1
    this.setData({
      num: this.data.num
    })
  },
  changeItemInArray: function() {
    this.setData({
      'array[0].text':'changed data'
    })
  },
  changeItemInObject: function(){
    this.setData({
      'object.text': 'changed data'
    });
  },
  addNewField: function() {
    this.setData({
      'newField.text': 'new data'
    })
  }
})
```

## 页面分享配置

**onShareAppMessage(Object object)**<br>
当用户点击页面内转发按钮（ttml中使用`button`组件，设置属性`open-type="share"`），或者点击右上角菜单`“分享”`按钮时，会回调该方法，用户可以在该方法内自定义分享内容。
### 参数
|名称|类型|描述|
|-----|----|----|
|from|String|转发事件来源。`button`：页面内转发按钮；`menu`：右上角转发菜单。|


### 自定义分享内容
此事件处理函数需要 return 一个 Object，用于自定义转发内容，返回内容的结构如下
|名称|类型|是否必填|描述|默认值|
|-----|----|----|----|----|
|title|String|必填|分享标题|当前小程序名字
|path|String|必填|移动端打开小程序加载的页面，如果不支持移动端需传空字符串('')，否则会默认为当前分享页面的路径|小程序当前页面|
|PCPath|String|选填|PC端打开小程序加载的页面，不支持可传空字符串|无|
|PCMode|String|选填|PC端打开小程序加载的模式，若需要在PC端打开小程序，则必须传PCMode字段|无|
|imageUrl|String|选填|自定义分享图,目前仅支持网络图片路径，图片大小建议1160x720 |小程序当前页面从顶部向下截取210x130作为分享图|
|success|function|选填|分享成功的回调|无
|fail|function|选填|分享失败的回调|无
::: note
`success`，`fail` 回调函数仅支持3.7.0+版本，依赖用户登录
:::

<br>
**success的返回值res里有个`data`数据**
|名称|类型|描述|
|-----|----|----|
|data|array|选择会话列表|

<br>
**data数组的每个数据都是一个Object类型，结构如下**
|名称|类型|描述|
|-----|----|----|
|id|string|openChatId|
|chatType|string|会话类型：0单聊,1群聊|

<br>
**示例代码**
```js
Page({
  onShareAppMessage: function (opt) {
      console.log(opt);
      return Object.assign({}, this.data.shareData, {
        title: opt.from === 'button' ? 'button share' : 'menu share',
        path: '/page/API/pages/share/share?a=b&from=' + opt.from,
        PCPath: '/page/API/pages/share/share?a=b&from=' + opt.from,
        PCMode: 'sidebar-semi',
        success (res) {
          console.log('success', res)
        },
        fail (err) {
          console.error(err);
        }
      })
    }
})
```

## 选区事件

:::note
版本要求：Lark ≥ 4.2
:::

监听 text 组件选区改变的事件，回调参数为一个 Object
|名称|类型|描述|
|---|---|---|
|content|string|选中的文本|
|isCollapsed|boolean|是否有选区|
|position|{x: number, y: number, width: number, height: number}|选区坐标位置|
|selectedObject|Array<{startOffset: number, endOffset: number, text: string, id?: string, dataset?: Object}>|当 text 组件嵌套的时候会返回每个 text 组件选中的信息|

**示例代码**
```js
Page({
  onSelectionChange(evt) {
    console.log(evt.content)
    if (evt.isCollapsed) {
      // 有选区状态，可以认为开始选择文本
    } else {
      // 无选区状态，可以认为取消选择文本
    }
  }
})
```

