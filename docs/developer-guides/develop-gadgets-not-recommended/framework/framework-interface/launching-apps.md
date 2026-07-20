---
document_id: '6965379541104885765'
directory_id: '6907567266540404738'
title: 小程序App
full_path: /uYjL24iN/uMDNuMDNuMDN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- Framework Interface
- Launching apps
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:37Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMDNuMDNuMDN
---

#  小程序App

## App(Object params)
App(params) 是框架启动小程序的入口函数, 开发者可以通过 App(params) 的参数指定小程序的生命周期函数和其他一些自定义参数。
> App(Object params) 必须在 app.js 中调用，并且只能调用一次，否则会出现不符合预期的结果

## 参数说明

|属性|类型|必填|描述|触发时机|
|---|---|----|-----|-----|
|onLaunch|function|否|生命周期函数--监听小程序初始化|当小程序初始化完成时，会触发 onLaunch（全局只触发一次）。参数也可以使用 [tt.getLaunchOptionsSync](/document/uYjL24iN/uAzM1YjLwMTN24CMzUjN) 获取。|
|onShow|function|否|生命周期函数--监听小程序显示|当小程序启动，或从后台进入前台显示，会触发 onShow。|
|onHide|function|否|生命周期函数--监听小程序隐藏|当小程序从前台进入后台，会触发 onHide。|
|onError|function|否|错误监听函数|当小程序发生脚本错误，或者 API 调用失败时，会触发 onError 并带上错误信息。|
|onPageNotFound|function|否|页面不存在监听函数|当小程序出现要打开的页面不存在的情况，会带上页面信息回调该函数，详见下文。|
|onThemeChange|function|否|监听系统主题色变化|当系统主题色发生变化时触发，同 [tt.onThemeChange]([onThemeChange](/document/uYjL24iN/uUTOuUTOuUTO/darkmode/onthemechange)) 一致。|
|其他|Any|否|开发者可以添加任意的函数或数据到 `Object `参数中，用` this `可以访问。|

**示例代码**

```js
App({
  onLaunch: function(options) {
    // Do something initial when launch.
  },
  onShow: function(options) {
      // Do something when show.
  },
  onHide: function() {
      // Do something when hide.
  },
  onError: function(msg) {
    console.log(msg)
  },
  onThemeChange: function({ theme }) {
    console.log(`Current theme is ${theme}`);
  },
  globalData: 'I am global data'
})
```

## onPageNotFound

当要打开的页面并不存在时，会回调这个监听器，并返回以下信息：

|属性|类型|说明|
|-----|---|-----|
|path|string|不存在页面的路径|
|query|Object|打开不存在页面的 query|
|isEntryPage|boolean|是否本次启动的首个页面|

示例代码：

```js
App({
  onPageNotFound(res) {
    tt.redirectTo({
      url: 'pages/...'
    }) // 重定向页面；如果是 tabbar 页面，请使用 tt.switchTab
  }
})
```

## getApp()

获取小程序全局唯一的 App 实例。可以获取在App() 设置的全局变量，或者一些通用方法

示例代码：

```js
var appInstance = getApp()
console.log(appInstance.globalData) // I am global data
```
