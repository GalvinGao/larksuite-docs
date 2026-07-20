---
document_id: '6965379543683121158'
directory_id: '6907567266536996865'
title: 注册页面
full_path: /uYjL24iN/ucjNzUjL3YzM14yN2MTN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- Logic Layer
- Page registration
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:19Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucjNzUjL3YzM14yN2MTN
---

# 注册页面
对于小程序中的每个页面，都需要在页面对应的 js 文件中进行注册，指定页面的初始数据、生命周期回调、事件处理函数等。
## 使用 Page 构造器注册页面
使用 Page() 进行构造。

**代码示例：**

```js 
//index.js
Page({
  data: {
    text: "This is page data."
  },
  onLoad: function(options) {
    // 页面创建时执行
  },
  onShow: function() {
    // 页面出现在前台时执行
  },
  onReady: function() {
    // 页面首次渲染完毕时执行
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
  },
  onPageScroll: function() {
    // 页面滚动时执行
  },
  // 事件响应函数
  viewTap: function() {
    this.setData({
      text: 'Set some data for updating view.'
    }, function() {
      // this is setData callback
    })
  }, 
```

页面生命周期的说明可以参考 [页面生命周期](/document/uYjL24iN/ugjNzUjL4YzM14CO2MTN)
详细的参数含义和使用请参考 [小程序页面](/document/uYjL24iN/uQDNuQDNuQDN)。
