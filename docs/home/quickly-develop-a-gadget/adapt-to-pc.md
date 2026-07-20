---
document_id: '7074246291802767365'
directory_id: '7002892512470728710'
title: 适配PC端
full_path: /home/develop-a-gadget-in-5-minutes/adapt-to-pc
breadcrumb:
- Home
- Quickly develop a gadget
- Adapt to PC
document_type: GuideDocumentType
updated_at: 2024-02-08T06:42:25Z
source_url: https://open.larksuite.com/document/home/develop-a-gadget-in-5-minutes/adapt-to-pc
---

# 适配 PC 端

Lark小程序支持在移动端和PC端运行：
- 你可以选择直接使用移动端的小程序代码在PC端运行，也可以为你的应用在PC端开发独立的小程序。
- 在PC端，小程序支持小窗口和大窗口两种模式。

## 小窗口模式


打开已经开发完成的小程序项目，在开发工具左上角单击 **真机调试** 功能并选择 **PC端** ，然后选择PC端模式为 **window-semi** 模式。

:::note
更多配置说明，可参考 [PC小程序](/document/uYjL24iN/uIjNzUjLyYzM14iM2MTN)。
:::


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2e4f85c43e5f3d4fd9bcef2eb8ee77d0_UMoodidtGG.png?height=886&lazyload=true&maxWidth=400&width=1462)


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/bef0cdb1f5a38818cd808aa018710823_w0QYNg96JJ.png?height=812&lazyload=true&maxWidth=600&width=1616)


## 大窗口模式

打开已经开发完成的小程序项目，在开发工具左上角单击 **真机调试** 功能并选择 **PC端** ，然后选择PC端模式为 **AppCenter** 模式。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9666896fc4f509e37fd62f7f5e76b4b2_EbmREdASSL.png?height=450&lazyload=true&maxWidth=400&width=888)

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/026b8671dc67e6c8d95b3a3ae21cda96_n8phXLCr3X.png?height=728&lazyload=true&maxWidth=600&width=1604)

如果你认为当前的确定按钮样式太长了，希望能够定义得小一些，你可以选择单独撰写小程序在PC打开时对应的页面，那么就可以为宽屏做特殊的样式定制。

**定制方法如下**：
:::html
<md-td>
1. 将`app.json`文件替换为以下内容，并保存。

	`ext`节点中单独为PC端定义了新的首页`pages/PC/pc-index`。
    ```json 
    {
      "pages":[
        "pages/index/index",
        "pages/PC/pc-index"
      ],
      "window":{
        "backgroundTextStyle":"light",
        "navigationBarBackgroundColor": "#fff",
        "navigationBarTitleText": "小程序",
        "navigationBarTextStyle":"black"
      },
      "ext": {
        "defaultPages": {
          "sidebarMode": "pages/index",
          "PCMode": "pages/PC/pc-index"
        }
      }
    }

    ```
2. 创建以下文件。
	-  `pages/PC/pc-index.ttml`
	-  `pages/PC/pc-index.js`
	-  `pages/PC/pc-index.ttss`
    :::note
    PC端小窗口模式仍然沿用 移动端模式的首页配置，即`"sidebarMode": "pages/index"`
    :::

    <img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0fa3729d5fcc36554b2a6242b984c2ba_to3fWefoFC.png?lazyload=true&width=1640&height=1264" style="width:70%"/>

3. 将之前写的 `index.js` `index.ttml` `index.ttss`中的内容对应复制到`pc-index.js` `pc-index.ttml` `pc-index.ttss`文件中，然后保存文件。
  
4. 打开`pc-index.ttss`文件，将按钮的样式调整为以下代码，将button组件的宽度设定为100px：

    ```css 
    .personal-sign-button {
      margin-top: 25px;
      width: 100px;
    } 
    ``` 
5. 再次在开发工具左上角单击 **真机调试** 功能并选择 **PC端** ，然后选择PC端模式为 **AppCenter** 模式。
  
	如下图所示，PC端的按钮效果已调整。

    <img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/718606a9aaabbf0ea0cd285e1e69f282_i6hLR8jKBL.png?lazyload=true&width=1640&height=831" style="width:70%"/>
</md-td>
:::



## 更多

Lark也提供了独立为PC小程序写一个代码程序包的方式。只需要在 [开发者后台](https://open.larksuite.com/app) 单独为PC小程序上传示例代码包即可。更多用法参见 [PC 小程序](/document/uYjL24iN/uIjNzUjLyYzM14iM2MTN)。


