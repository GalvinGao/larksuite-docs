---
document_id: '6965379543683104774'
directory_id: '6907567269107695618'
title: 小程序宿主环境
full_path: /uYjL24iN/uUDNzUjL1QzM14SN0MTN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Introduction
- Host Environment
document_type: GuideDocumentType
updated_at: 2025-02-20T06:18:28Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUDNzUjL1QzM14SN0MTN
---

# 小程序宿主环境

::: warning
[小程序](/document/uYjL24iN/uUDNzUjL1QzM14SN0MTN)能力将不再迭代，推荐选择[网页应用](/document/uYjL24iN/uMTMuMTMuMTM/introduction)能力。<br>

从25年3月3日起，针对 **尚未** 开发过小程序应用的企业，将 **不允许** 创建小程序的入口。<br>

针对已开发过小程序能力的企业仍然可以创建小程序，不受影响。
:::

Lark小程序是运行在Lark客户端上(包括PC端和移动端)，客户端为小程序提供运行时所需的环境和各种能力，支撑起小程序应用。我们将这套环境称为**宿主环境**。通过宿主环境提供的各种能力，小程序可以实现很多Web网页无法完成的功能，获得更流畅的体验和交互。

## 逻辑层和视图层

小程序的运行环境分为2部分：逻辑层和视图层。其中逻辑层是加载小程序应用的JS脚本，负责处理业务逻辑的。而视图层负责渲染TTML模板和TTSS样式，显示页面。<br>

小程序的逻辑层和视图层是2个独立的线程来管理。逻辑层的线程启动JSCore引擎去执行JS脚本，处理业务逻辑。视图层的每个页面使用一个Webview进行渲染，多个页面则是多个Webview。 所以一个小程序应用只有一个JSCore线程，多个Webview线程。<br>

小程序整体的运行环境和通信模型如下图所示

![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/39d8eb5e9157efbf1050aec927f00f1f.png?lazyload=true&width=1640&height=608)

小程序的宿主环境就是图中的Mobile或者PC客户端，分别跟视图层和逻辑层进行通信和数据交互。视图层和逻辑层之间无法直接通信，只能通过客户端作为桥来通信。


## 程序与页面

在Lark里打开小程序时，首先会将小程序的代码包下载到本地，然后解析app.json里的pages字段，就可以知道该小程序的所有页面路径：
```javascript
{
  "pages":[
    "pages/index/index",
    "pages/home/home"
  ]
}
```
pages里配置的第一个值，就是小程序应用的打开时看到的第一个页面。<br>

每个页面路径在程序代码包里都可以找到对应的文件夹路径。例如pages/index/index，在该路径下，存在着这个页面对应的TTML、TTSS和JS文件。小程序启动之后，就根据这个路径找到对应页面的这些代码文件，然后交给逻辑层和视图层执行，于是我们就可以在客户端上看到该页面。<br>

当小程序启动之后，会触发 app.js 里定义的**onLaunch**方法
```javascript
App({
  onLaunch: function () {
    // 小程序启动之后执行
  }
})
```
> 注意：移动端的一个小程序只存在一个实例，而PC端的同一个小程序的每个模式下会存在唯一的实例（PC小程序有多种模式，详见[PC小程序运行模式](/document/uYjL24iN/uIjNzUjLyYzM14iM2MTN#37991f21)）。

下面我们具体看下小程序的页面结构和具体的构成，以pages/index/index为例，该文件目录下存在
index.js，index.ttml，index.ttss。
```javascript
// index.js 的内容
Page({
  data: {

  },
  onLoad: function () {
    console.log('Welcome to Mini Code')
  },
})
```
在index.js文件里 Page({...}) 就构造出这index页面实例，data表示的是该页面需要用到的渲染数据。
当生成页面时，小程序引擎会把 data 数据和 index.ttml 结合，再渲染出来显示给用户。<br>

onLoad() 方法则是页面的生命周期方法，页面创建时会回调该方法，开发者可在里面定义自己的逻辑。


## 组件
组件是小程序提供给开发者来创建页面UI，自定义TTML的。开发者可以以灵活的方式自由组合各种组件，去构建属于自己的页面UI。<br>

**什么是组件：**
- 在TTML中使用组件，是视图层的基本组成单元。
- 一个组件通常包括开始标签和结束标签，属性用来修饰这个组件，内容在两个标签之内。
> 注意：所有组件与属性都是小写，以连字符-连接

就像HTML的标签一样，在小程序里，开发者在TTML里写上对应组件的标签，就能显示出来。例如：
```javascript
// index.ttml 内容
<view class="intro">Welcome to Gadget</view>
```
该页面就显示一行文案： Welcome to Gadget。开发者可以通过 **style** 和 **class** 来控制该组件的样式，来指定宽、高、颜色等

更多组件相关的内容可以参考 [小程序的组件](/document/uYjL24iN/ugTNugTNugTN)。


## API  
为了让开发者很方便的使用Lark提供的能力，例如获取用户信息，获取设备当前的地理位置，播放视频等等。小程序提供了很多API供开发者使用，这些API分为两种：同步、异步。其中异步 API 通过回调返回结果，同步 API 直接返回结果。

例如获取用户信息:
```javascript
tt.getUserInfo({ 
    success (res) { 
        console.log(`getUserInfo 调用成功 ${res.userInfo}`); 
    }, 
    fail (res) { 
        console.log(`getUserInfo 调用失败`); 
    } 
});
```
获取设备当前的地理位置：
```javascript
tt.getLocation({ 
    success (res) { 
        console.log(`经度 ${res.longitude}，纬度 ${res.latitude}`); 
    }, 
    fail (res) { 
        console.log(`getLocation 调用失败`); 
    } 
});
```
> 很多API都是基于异步回调的，所以需要开发者处理好异步逻辑，传入正确的回调函数去处理业务

更多API相关的内容可以参考 [小程序的API](/document/uYjL24iN/uADOy4CM4IjLwgjM)。
