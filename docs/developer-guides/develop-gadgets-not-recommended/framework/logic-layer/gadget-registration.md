---
document_id: '6965379543683252230'
directory_id: '6907567266536996865'
title: 注册小程序
full_path: /uYjL24iN/uYjNzUjL2YzM14iN2MTN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- Logic Layer
- Gadget registration
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:17Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYjNzUjL2YzM14iN2MTN
---

# 注册小程序
小程序注册的逻辑在 app.js 中，在 App 对象中可以实现对小程序生命周期函数的监听，可以配置全局共享的数据。

详细的参数含义和使用请参考 [小程序App](/document/uYjL24iN/uMDNuMDNuMDN)。

```js 
// app.js
App({
  onLaunch (options) {
    // Do something initial when launch.
  },
  onShow (options) {
    // Do something when show.
  },
  onHide () {
    // Do something when hide.
  },
  globalData: 'I am global data'
}) 
```


 getApp 方法可以在任何页面内获取 App 实例，整个小程序只有一个 App 实例，是全部页面共享的，通过 getApp 方法获取App实例后，可以访问 App 的数据或调用开发者注册在 App 上的函数。

```js 
// xxx.js
const appInstance = getApp()
console.log(appInstance.globalData) // I am global data 
```
