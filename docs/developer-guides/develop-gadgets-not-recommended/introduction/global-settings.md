---
document_id: '6965379543684481030'
directory_id: '6907567269107695618'
title: 配置小程序
full_path: /uYjL24iN/uEDNuEDNuEDN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Introduction
- Global Settings
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:11Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEDNuEDNuEDN
---

# 全局配置

小程序根目录下的 `app.json` 文件用来对小程序进行全局配置，决定页面文件的路径、全局样式、设置多 tab，设置PC小程序各种模式的默认启动页面等。
以下是一个包含了部分常用配置选项的 `app.json` ：

```json
{
  "pages": [
    "pages/index/index",
    "pages/logs/index",
    "pages/home/index",
    "pages/help/index"
  ],
  "window": {
    "backgroundTextStyle":"light",
    "navigationBarBackgroundColor": "#fff",
    "navigationBarTitleText": "Mini App",
    "navigationBarTextStyle": "black"
  },
  "tabBar": {
    "list": [{
      "pagePath": "pages/home/index",
      "text": "主页"
    }, {
      "pagePath": "pages/help/index",
      "text": "帮助"
    }]
  }
}
```

## 全局配置项

|属性|类型|必填|描述|
|-----|---|-----|-----|
|entryPagePath|string|否|小程序默认启动页面|
|pages|string[]|是|配置页面路径|
|window|Object|否|配置默认页面的窗口表现|
|tabBar|Object|否|配置底部 tab 的表现|
|debug|Boolean|否|配置是否开启 debug 模式|
|ext|Object|否|提供额外配置参数给小程序使用|
|darkmode|Boolean|否|设置为 true 时表示小程序支持 DarkMode|
|themeLocation|String|否|变量配置文件的路径，相对于小程序根目录，darkmode 为 true 时为必填|
|useExtendedLib|Object|否|配置需要使用的扩展库|
如果你的小程序支持在PC端运行，可以在ext里配置PC小程序各种模式的默认启动页面。配置完成后，在启动PC小程序时如果不设置启动页面，将根据当前模式启动配置中的页面。具体示例如下：

```json
{
  "pages": [
    "pages/index/index",
    "pages/logs/index",
    "pages/order/index",
    "pages/chat/index"
  ],
  "ext": {
    "defaultPages": {
      "sidebarMode": "pages/order/index",
      "PCMode": "pages/chat/index"
    }
  }
}
```
>如果你的PC小程序需要配置默认启动页面，请注意：defaultPages中各模式配置的页面必须在pages中定义，否则编译过程中会报错。

### entryPagePath
指定小程序启动的默认页面，未配置，则默认为 pages 列表的第一项。
```
{
  "entryPagePath": "pages/index/index"
}
```

### defaultPages参数说明
|属性|类型|必填|描述|
|-----|---|-----|-----|
|sidebarMode|string|否|小窗口模式默认展示页面|
|PCMode|string|否|大窗口模式默认展示页面|

### pages

这个字段用于配置小程序用到的所有页面路径，配置每项是 `路径 + 文件名` 这个结构。**配置项的第一个页面路径就是小程序启动展示的第一个页面**。

**需要注意：保证单个页面的 `.json`，`.js`， `.ttml`，`.ttss` 资源都放在每个页面路径的首层**。

如果开发目录如下：

```
|____app.ttss
|____app.json
|____project.config.json
|____pages
|       |____index
|       |        |____index.js
|       |        |____index.json
|       |        |____index.ttml
|       |        |____index.ttss
|____app.js
```

那么 app.json 应该这样配置：

```json
{
  "pages":[
    "pages/index/index"
  ]
}
```

### window

这个字段用于设置小程序的状态栏、导航栏、标题、窗口背景色。

|属性|类型|默认值|描述|支持平台|
|-----|---|-----|-----|-----|
|navigationBarBackgroundColor|HexColor|#000000|导航栏背景颜色，如 "#000000"|iOS,Android|
|navigationBarTextStyle|String|white|导航栏标题颜色，仅支持 black/white|iOS,Android|
|navigationBarTitleText|String|-|导航栏标题文字内容|iOS,Android,PC|
|transparentTitle|String|none|导航栏透明设置。默认 none，支持 always 一直透明 / auto 滑动自适应 / none 不透明|iOS,Android|
|navigationStyle|String|default|导航栏样式，仅支持 default/custom。<br>custom 模式可自定义导航栏，只保留右上角胶囊状的按钮|iOS,Android,PC|
|backgroundColor|HexColor|#ffffff|窗口的背景色|iOS,Android,PC<br>(PC端3.14.0+)|
|backgroundTextStyle|String|dark|下拉 loading 的样式，仅支持 dark/light|iOS,Android|
|backgroundColorTop|String|#ffffff|顶部窗口的背景色|iOS|
|backgroundColorBottom|String|#ffffff|底部窗口的背景色|iOS|
|enablePullDownRefresh|Boolean|false|是否开启下拉刷新|iOS,Android|
|onReachBottomDistance|Number|50|页面上拉触底事件触发时距页面底部距离，单位为 px|iOS,Android|
|PCMode|Object|-|PCMode模式下特定的窗口配置，支持的属性与通用window配置属性一致，**仅当在ext内配置了defaultPages.PCMode时生效**|PC(3.14.0+)|


### tabBar

如果小程序是一个多 tab 应用（客户端窗口的底部或顶部有 tab 栏可以切换页面），可以通过 tabBar 配置项指定 tab 栏的表现，以及 tab 切换时显示的对应页面。

|属性|类型|必填|默认值|描述|支持平台|
|-----|---|-----|-----|-----|-----|
|color|HexColor|是||tab 上的文字默认颜色|iOS,Android,PC|
|selectedColor|HexColor|是||tab 上的文字选中时的颜色|iOS,Android,PC|
|backgroundColor|HexColor|是||tab 的背景色|iOS,Android,PC|
|borderStyle|string|否|black|tabbar 上边框的颜色， 仅支持 black/white|iOS,Android,PC<br>(PC端3.14.0+)|
|list|Array|是||tab 的列表, **最少 2 个, 最多 5 个 tab**|iOS,Android,PC|
|position|string|否|bottom|可选值 bottom/top|PC|
|PCMode|Object|否|-|PCMode模式下特定的tabBar配置，支持的属性与通用tabBar配置属性一致，**仅当在ext内配置了defaultPages.PCMode时生效**|PC(3.14.0+)|


其中 list 接受一个数组，**只能配置最少 2 个、最多 5 个 tab**。tab 按数组的顺序排序，每个项都是一个对象，其属性值如下：

|属性|类型|必填|描述|
|-----|---|-----|-----|
|pagePath|string|是|页面路径，必须在 pages 中先定义|
|text|string|是|tab 上按钮文字|
|iconPath|string|否|图片路径，icon 大小限制为40kb，建议尺寸为 96px * 96px，不支持网络图片|
|selectedIconPath|string|否|选中时的图片路径，icon 大小限制为 40kb，建议尺寸为 96px * 96px ，不支持网络图片|
> 注意：iconPath和selectedIconPath必须使用png图片

### useExtendedLib
配置需要使用的拓展库，目前支持以下库：

- [ChartSpace 图表组件库](/document/uYjL24iN/uUTM5UjL1ETO14SNxkTN/extension/visualization/chartspace) 

配置后，会将扩展库的最新版本 npm 包内置到小程序中。详细配置如下：
```json
{
  "useExtendedLib": {
    "chartSpace": true
  }
}
```
在使用自定义组件的页面 json 中增加自定义组件配置：
```
{
  "usingComponents": {
    "chart-space": "/lark-chartspace/index"
  }
}
```

## 页面配置

每一个小程序页面也可以使用同名` .json` 文件来对本页面的窗口表现进行配置，页面中配置项会覆盖 `app.json` 的 `window` 中相同的配置项。

`app.json`配置示例：

```json
{
  "navigationBarBackgroundColor": "#ffffff",
  "navigationBarTextStyle": "black",
  "navigationBarTitleText": "App",
  "backgroundColor": "#333333",
  "backgroundTextStyle": "light"
}
```

## 页面配置项

|属性|类型|默认值|描述|支持平台|
|-----|---|-----|-----|-----|
|navigationBarBackgroundColor|HexColor|#000000|导航栏背景颜色，如 "#000000"|iOS,Android|
|navigationBarTextStyle|String|white|导航栏标题颜色，仅支持 black/white|iOS,Android|
|navigationBarTitleText|String||导航栏标题文字内容|iOS,Android,PC|
|backgroundColor|HexColor|#ffffff|窗口的背景色|iOS,Android|
|backgroundTextStyle|String|dark|下拉 loading 的样式，仅支持 dark/light|iOS,Android|
|enablePullDownRefresh|Boolean|false|是否开启下拉刷新|iOS,Android|
|usingComponents|Object|否|页面自定义组件配置（具体使用可以参考文档[《自定义组件》](/document/uYjL24iN/ugTOugTOugTO)）|iOS,Android,PC|
|disableScroll|Boolean|false|设置为 true 则页面整体不能上下滚动；**只在 page.json 中有效，无法在 app.json 中设置该项**|iOS,Android|
|disableSwipeBack|Boolean|false|禁止页面右滑手势返回(**目前不支持在app.json、小程序首页配置**)| Android|
|onReachBottomDistance|Number|50|页面上拉触底事件触发时距页面底部距离，单位为 px|iOS,Android|

> 注意:页面配置中只能设置 app.json 中 window 对应的配置项，以决定本页面的窗口表现，所以无需写 window 这个属性。
