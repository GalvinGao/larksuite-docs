---
document_id: '6969541447007895557'
directory_id: '6907567269107695618'
title: 'PC 小程序 '
full_path: /uYjL24iN/uIjNzUjLyYzM14iM2MTN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Introduction
- PC Gadget
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:14Z
source_url: https://open.larksuite.com/document/uYjL24iN/uIjNzUjLyYzM14iM2MTN
---

# PC小程序
## PC小程序介绍
Lark小程序支持在移动端和PC端运行，你可以选择直接使用移动端的小程序代码在PC端运行，也可以为你的应用在PC端开发独立的小程序。
为了充分利用PC端更大的屏幕展示面积，在PC端，小程序支持小窗口和大窗口两种模式，打开场景也更加多样，你可以利用这些特性开发出能力更加强大的小程序应用。

## PC小程序代码包
我们同时支持PC端使用移动端小程序代码包和使用独立的小程序代码包，你可以根据你的研发习惯选择合适的方式。

- ### 移动端小程序包

默认情况下我们会在PC端使用你的移动端小程序代码包，你不需要进行任何的设置，只需要在PC端对你的小程序进行必要的功能测试即可。

使用移动端小程序包时，小程序会按照[PC小程序运行模式](/document/uYjL24iN/uIjNzUjLyYzM14iM2MTN#137a7878)中根据mode参数使用对应的模式打开。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/741a1a6bd736d540427f9926c83417ab_97YGkJLVRL.png)

- ### 不使用移动端小程序包

**当仅有移动端小程序包时**，可以选择为**不使用移动端小程序包**，当在PC端使用Applink等打开时，会以二维码的方式引导用户使用移动端扫码打开应用。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5719932b6ff8ee3d43bdc398ebb1f091_vIh5GsHB3u.png)

- ### PC端独立小程序包
如果你希望独立维护移动端和PC端的小程序代码，你也可以选择为PC端小程序上传独立的代码包。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/453e6798bd4593ee3024e3cd6e806f1d_WWV4LTbWl6.png)

当然，如果你希望在同一个工程里开发移动端和PC端小程序，你也可以选择仅上传一份代码到移动端小程序包、在PC端保持使用移动端小程序代码包。两种方式在结果上没有差异，你可以根据你或者团队的研发工作习惯选择合适的方式。


## PC小程序运行模式
### 小窗口
小程序在PC端默认支持小窗口模式，在小窗口模式下运行效果与在移动端比较接近，如下图所示：



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/51a87e4bd6e471e3554d7ac6cfdf3068_y3quwv9Eh1.png)

### 大窗口
除了小窗口模式，小程序在PC端还支持大窗口模式，大窗口模式的运行效果如下：


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/51ce575f27232c405dbce6820808562d_tXWebLrrU6.png)

要支持大窗口模式，只需要按照小程序的[全局配置说明](/document/uYjL24iN/uEDNuEDNuEDN#top_anchor)，通过app.json文件“ext > defaultPages”中的“PCMode”配置大窗口模式的默认启动页面即可。


```js 
{
  "pages": [
    "pages/index/index",
    "pages/logs/index",
    "pages/order/inde",
    "pages/chat/index"
  ],
  "ext": {
    "defaultPages": {
      "sidebarMode": "pages/component/index",//小窗口模式启动小程序时的默认启动页面
      "PCMode": "pages/PC/component/index"//大窗口模式启动小程序时的默认启动页面
    }
  }
} 
```
在启用了大窗口模式的情况下，在PC端通过“工作台”启动小程序时将默认以大窗口打开小程序。

除此之外，也可以[通过Applink打开小程序](/document/uYjL24iN/ucjN1UjL3YTN14yN2UTN#2f51bbd)。在使用Applink打开小程序时，可以通过设置链接中的mode参数来控制小程序的启动模式：当mode=sidebar-semi时，将以小窗口打开小程序（对应下表两类场景）；当mode=appCenter时，将以大窗口打开小程序。

**不同的 mode 参数值在不同情况下的打开效果如下：**

| mode 参数值         | 不同场景下的启动形式        | 图示      | 
| --------- | --------------- | -------   | 
|mode=sidebar-semi | 场景1：在IM 聊天页面启动时，会以侧边栏的形式打开 |![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6729e3ec2ba47aca5cf532c83cee5078_3vJoyEgGUF.png)| 
|mode=sidebar-semi|场景2：在 IM 聊天页面之外的场景启动时，会以独立小窗口的形式打开|![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/51a87e4bd6e471e3554d7ac6cfdf3068_AMMYMEcXsb.png)|
|mode=window-semi| 会以独立小窗口的形式打开|![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/51a87e4bd6e471e3554d7ac6cfdf3068_uKLFfGEKT8.png)|
|mode=appCenter|会以工作台 tab 的形式打开|![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/51ce575f27232c405dbce6820808562d_LwuOTyqw3x.png)|
|mode=window|会以独立大窗口的形式打开|![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/05c39c181e11f7f992dbd4d9c95e18a0_gfkyiEQkv4.png)|





