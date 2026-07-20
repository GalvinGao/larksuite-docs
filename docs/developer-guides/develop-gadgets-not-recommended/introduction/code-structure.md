---
document_id: '6965379541104230405'
directory_id: '6907567269107695618'
title: 小程序代码构成
full_path: /uYjL24iN/uQDNzUjL0QzM14CN0MTN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Introduction
- Code Structure
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:08Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQDNzUjL0QzM14CN0MTN
---


# 小程序代码构成
## 代码组成和结构

查看小程序的项目，我们可以看到项目代码里包含了下面几种格式的文件：
1. .json 后缀的 JSON 配置文件
2. .ttml 后缀的 TTML 模板文件
3. .ttss 后缀的 TTSS 样式文件
4. .js 后缀的 JS 脚本逻辑文件


接下来我们分别看看这 4 种文件的作用。

### JSON配置

我们可以看到在项目的根目录有一个 app.json 和 一个project.config.json，下面我们依次来说明它们的用途。

**（1） 小程序配置 ​app.json​**

app.json 是当前小程序的全局配置，包括了小程序的所有页面路径、页面样式配置、头部 title、底部 tab 等。用开发工具生成的项目里面包含一个默认的 app.json 文件，通过该文件可以配置一些小程序全局的属性，声明小程序所有的页面，同时，如果小程序可以在PC端运行，还能在这个文件里配置默认启动的页面等。

**（2）项目配置 ​project.config.json​**

项目配置文件里主要包括了针对小程序项目配置的一些信息，例如项目名称，App ID，项目语法，编译配置等内容。这些内容可以在开始创建项目的过程中通过开发者工具生成，开发者也可以根据需要进行修改和配置。

JSON配置文件的细节和具体参数可以查看文档[《配置小程序》](/document/uYjL24iN/uEDNuEDNuEDN)。

### TTML模版

TTML 是一种用来描述当前页面的结构，充当类似 HTML 的角色。下面是一个简单的ttml文件：

```html
<view class="container">
  <view class="user" tt:if="{{hasUser}}">
    <text class="text">Hello World</text>
     <button bindtap="tap">Button</button>
  </view>
</view>
```

和 HTML 非常相似，TTML 由标签、属性等构成。但是也有一些不同的地方：
1. 标签名字不一样。TTML提供了 view, button, text 等等，这些标签就是小程序给开发者包装好的基本能力，我们还提供了地图、视频、音频等等组件能力。
2. 多了一些 tt:if 这样的属性以及 {{ }} 这样的表达式。
3. TTML作为模版语法来描述状态和界面结构的关系，实现渲染和逻辑的分离。
4. 小程序的框架也是用到了这个思路，举个简单的例子，如果你需要通过点击按钮显示 「Hello World」 在界面上。 TTML 是这么写：

```html
<view class="container">
  <view class="userinfo">
    <button bindtap="showMessage">点击按钮显示Message</button>
  </view>
  <view class="textinfo" tt:if="{{isShowMessage}}">
    <text class="text">{{message}}</text>
  </view>
</view>
```

JS 只需要管理状态即可：

```js
const app = getApp()
Page({
  data: {
    message: 'Hello World',
    isShowMessage: false,
  },
  showMessage() {
    this.setData({
      isShowMessage: true,
    })
  },
})
```

通过 {{ }} 的语法把一个变量绑定到标签上，我们称为数据绑定。仅仅通过数据绑定还不够完整的描述状态和页面的关系，还需要提供一些逻辑判断和循环遍历的功能，如 if / else, for 等能力，在小程序里边，这些控制能力都用 tt: 开头的属性来表达。

TTML文件的详细介绍可以参考文档[《TTML》](/document/uYjL24iN/ugzNugzNugzN)
### TTSS 样式

TTSS 具有 CSS 大部分的特性，小程序的 TTSS 也做了一些扩充和修改。
1. 新增了一个动态的尺寸单位。
2. 在写 CSS 布局时，样式的展示会受设备的屏幕宽度和设备像素影响，因此我们会采用一些技巧来换算一些像素单位。TTSS 在底层支持新的尺寸单位 rpx ，开发者可以免去换算的烦恼，只要交给小程序底层来换算即可，由于换算采用的浮点数运算，所以运算结果会和预期结果稍微会有些许偏差。
3. 提供了全局的样式和局部样式。
4. 和前边 app.json, page.json 的概念相同，你可以写一个 app.ttss 作为全局样式，会作用于当前小程序的所有页面，局部页面样式 page.ttss 仅对当前页面生效。
5. 此外 TTSS 仅支持一部分的 CSS 选择器。

TTSS文件的详细介绍可以参考文档[《TTSS》](/document/uYjL24iN/uYDOuYDOuYDO)

### JS 交互逻辑

一个小程序不仅需要页面，还需要和用户做交互：响应用户的点击、获取用户的位置等等。在小程序里面，我们就通过提供 ​JS​ 文件来响应用户的操作。

```html
<view class="container">
  <view class="model">
    <text class="info">{{message}}</text>
    <button bindtap="showModel">显示设备型号</button>
  </view>
</view>
```

点击 ​button​ 按钮的时候，我们希望显示用户当前手机的型号，于是我们在 ​button​ 上声明一个属性: ​bindtap​ ，在 JS 文件里边声明了 ​showModel​ 方法来响应这次点击操作：

```js
const app = getApp()
Page({
  data: {
    message: '',
  },
  showModel() {
    // 获取设备信息
    tt.getSystemInfo({
      success: (res) => {
        // 调用this.setData可以进行状态管理
        this.setData({
          message: res.model,
        })
      },
    })
  }
})
```

处理用户的操作就变得很简单明了了。

此外开发者还可以在 JS 中调用客户端为小程序提供的丰富的 API，利用这些 API 可以很方便的调用客户端提供的能力，例如获取用户信息、本地存储、定位等。
