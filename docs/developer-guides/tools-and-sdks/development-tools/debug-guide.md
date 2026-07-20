---
document_id: '6967331158355705862'
directory_id: '6907567269107613698'
title: 调试指南
full_path: /uYjL24iN/ugDOzYjL4gzM24CO4MjN
breadcrumb:
- Developer Guides
- Tools and SDKs
- Development Tools
- Debug Guide
document_type: GuideDocumentType
updated_at: 2021-05-28T13:23:18Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDOzYjL4gzM24CO4MjN
---

# 调试指南

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;程序调试主要有两个大功能区：**模拟器** 和 **调试器**。

## 模拟器

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;在开发者工具中模拟小程序在LarkApp中的展现效果，对于绝大部分的 API 均能够在模拟器上呈现出准确的视觉状态。

![simulator.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/54899915438f9b48cb6117b5c0f88291_fBYtLfRZg5.png)

### 顶部工具栏
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;开发者可以在调试器中切换模拟设备、调整窗口大小，也可以模拟移动端设备的旋转，可用于适配典型常见机型的模拟。

![rotate.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2a2c690dd57851fc4ecc086182d0b86b_u112jnPQpx.png)

#### 工具箱中可以模拟的设备操作

- 前后台切换  
- [位置](/document/uYjL24iN/uUTOz4SN5MjL1kzM)  
- [扫码](/document/uYjL24iN/uYzNx4iN3EjL2cTM)  
- [权限](/document/uYjL24iN/uITMuITMuITM)  
- [截屏](/document/uYjL24iN/uMjNwEjLzYDMx4yM2ATM)  
- [罗盘](/document/uYjL24iN/uIzNx4iM3EjLycTM)  
- [Wi-Fi](/document/uYjL24iN/uYDO4UjL2gDO14iN4gTN)  
- [加速度计](/document/uYjL24iN/ukjNx4SO2EjL5YTM)

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/275bde5225cbb936afc793ce956e3421_Q1dviebHkX.png)


 &nbsp;  
#### 模拟API接口调用  


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9e72919b2914548f32b0452b9145e15d_kQwsFIevuZ.png)

- 打开用户联系人选择列表: [chooseContact](/document/uYjL24iN/uMTM04yMxQjLzEDN) 
- 打开用户会话列表选择会话: [chooseChat](/document/uYjL24iN/uMTN3QjLzUzN04yM1cDN)  
- 调起二次验证安全密码输入界面: [startPasswordVerify](/document/uYjL24iN/ugTO3IjL4kzNy4CO5cjM)   
- 获取bdp_launch_query值： [getHostLaunchQuery](/document/uYjL24iN/ugzM4UjL4MDO14COzgTN)  
- 获取某个会话的信息：[getChatInfo](/document/uYjL24iN/uEDN2UjLxQjN14SM0YTN)


####  模拟触发未读消息：[onChatBadgeChange](/document/uYjL24iN/uQDN2UjL0QjN14CN0YTN)

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1f061408457a23289c43c2a83ce12d32_mpOIGhaY7K.png)


### 底部状态栏

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;底部工具栏部分可切换显示页面路径、页面参数和场景值。同时通过勾选“自动刷新”可以在保存代码时自动编译项目并刷新模拟器中的小程序。


![status_bar.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dafa4ed26b94658c840d2688c970b70b_MoZVIdPCeZ.png)

## 调试工具

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;调试工具分为 8 大功能模块，分别是 **Elements**、**Console**、**Sources**、**Network**、**Appdata**、**Storage**、**Audit**、**Log**。

### Elements panel

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Elements panel 的使用方式和 Chrome 调试器中的 Elements 没有区别，可用于帮助开发者开发 ttml 转化后的界面。在这里可以看到真实的页面结构以及结构对应的 ttss 属性，同时可以通过修改对应 ttss 属性，在模拟器中实时看到修改的情况（仅为实时预览，无法保存到文件）；通过调试模块左上角的选择器，还可以快速定位页面中组件对应的 ttml 代码。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/35cf2ea527dfb6620734e2140866ec42_C8xDQhytDs.png)


### Console panel

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Console panel 有两大功能：

- 开发者可以在此输入和调试代码

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c05a61e739582656d4c6369f586094d9_mdE0AdZVLR.png)


- 小程序的 console 和 error 输出，会显示在此处

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/075c3a90b8a525cd102ed5ba8a8f6b8d_65UbdyfoPk.png)

### Sources panel

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Sources panel 用于显示当前项目的脚本文件，同浏览器开发时显示的 Sources 面板基本没有区别。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/851acf5f5ccf275f3eae76448914cfe6_ELj8h95N3W.png)



>注：当代码运行到断点的时候，整个小程序都停止了，会有明显的 debugger 提示

### Network panel

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Network Panel 用于观察和显示 request 和 socket 的请求情况

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/48b3994ed7c29e819c5c46c724c2852a_n9ElbOuEfz.png)


### AppData panel

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;AppData panel 用于显示当前项目运行时小程序 AppData 具体数据，实时地反映项目数据情况，可以在此处编辑数据，并及时地反馈到界面上。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7fced0f86844b53db336f834eae86999_jbN5YFc52N.png)


### Storage panel

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Storage panel 用于显示当前项目使用 ==tt.setStorage== 或者 ==tt.setStorageSync== 后的数据存储情况，开发者可以直接在 Storage panel 上对数据进行删除（按 delete 键）、新增、修改。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d5db045e4aceb6c2544020ff50f04961_KG02fUQDeQ.png)


### Audit Panel

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Audit Panel 用于给小程序的体验评分，它会在小程序运行过程中实时检查，分析出一些可能导致体验不好的地方，并且定位出哪里有问题，以及给出一些优化建议。

#### 使用流程

1. 点击“运行”按钮，然后自行操作小程序界面，运行过的页面就会被“体验评分”检测到。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b117f36dc99c098d68b0474a44ea2ca3_9lgqpHD57Q.png)

2. 点击 “停止” 则结束检测，在当前面板显示相应的检测报告，开发者可根据报告中的建议对相应功能进行优化；开发者可点击“已覆盖页面”和“未覆盖页面”查看当前评分的页面覆盖情况。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9bc2150930fe5be3884c1e8a7465b608_QnKkIz5r6p.png)


3. 如需保存当前体验评分，可点击报告上方的“导出”按钮，将报告导出到本地保存；如需再次运行体验评分，可点击“清空体验评分”恢复初始状态。
   > 点击[体验评分方法及规则](/document/uYjL24iN/ukDOzYjL5gzM24SO4MjN) 文档查看具体的评分细则。

### Mock Panel

> 支持版本：v1.2.0  
为了让开发者更方便地开发小程序，开发者工具提供了 API Mock 的能力，可以模拟部分 API 的调用结果。

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;首先你需要在左侧面板上方勾选“启用 Mock”打开 Mock 功能，可以点击加号新增一条规则，点击减号删除一条规则，双击规则的名字可以编辑规则名称。

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;在右侧面板，你可以设置当前规则的 Mock 信息，支持以下 API 以及大部分输入继承自[标准输入对象](/document/uYjL24iN/ukzNy4SO3IjL5cjM)的异步 API：
```js
tt.request
tt.downLoadFile
```

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;当 Mock 功能启用时，在模拟器中调用的 API 时会按照以下方式从上往下命中第一条规则：
1. 规则启用
2. API 接口名匹配
3. 该规则下设置的所有的参数都能通过正则表达式
4. 不填写规则会默认匹配

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;命中后，该 API 按照命中规则的模拟返回输出，否则不进入 Mock 的逻辑。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f47cbfd8b9a126d995bcf75221018f7f_7GW0HNNp73.png)


### Log Panel

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Log Panel 用于查看模拟器运行时，工具内部的日志信息。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/edd39664c7f9ef5834648d416556c444_CU0tjsK6eL.png)


## 真机实时预览

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;真机实时预览可以实现编写小程序时快速预览，免去了每次查看小程序效果时都要扫码的麻烦。

> 真机实时预览功能依赖局域网通信，需要保证当前设备和手机在同一网络下
>
> 需要移动端Lark版本 >= 3.12.0

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;在终端运行以下命令，即可开启实时预览功能

```js
$ opdev preview [项目路径] -t gadget -p mobile -w
```

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;其中项目路径为小程序代码所在位置，如果小程序依赖预编译框架（如：uni-app、taro 等），则需设置为编译后的文件夹路径。

### 扫码预览

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;用Lark扫描二维码即可在移动端预览小程序

![图片名称](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/1920367163a826826e2785820d9aedc9.png)

### 代码变动监听

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;真机实时预览功能支持监听代码的修改，当代码发生修改并保存后，开发者工具会实时编译并将编译后的小程序推送给Lark移动端，即可预览修改后的小程序。

## H5 应用调试
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;首先通过预览命令打开Lark工作台预览 H5 应用
```shell
opdev preview [H5_app_path] --type=h5
```
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;如果是本地项目，修改项目代码，保存后刷新页面就能看到效果。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0b0c21f9ea90e02aa1ba6d780589b1de_vPX6NzqHLP.png)

## 真机调试
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;真机远程调试功能可以实现直接利用开发者工具，通过连接，对手机、iPad 和 PC 上运行的小程序进行调试，帮助开发者更好的定位和查找在真机上出现的问题。
>真机实时预览功能依赖局域网通信，需要保证当前设备和手机在同一网络下
>
>PC端需要在同一台设备上保持Lark打开
>
>需要移动端或PC端Lark版本 >= 3.45.0
>
>建议node 版本为 v12.5.0

### 发起调试
要发起一个真机远程调试流程，需要先点击开发者工具的工具栏上 “真机调试” 按钮。

![remote-debug01.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0a00796981c47bfac52fb85978d50a13_wBxv6PCDlI.png)

点击之后默认会打开移动端调试，此时，工具会将本地代码进行处理打包，就绪之后，使用手机客户端扫描二维码即可弹出调试窗口，开始远程调试。

目前工具也支持PC端调试，打包流程同移动端类似，处理完毕之后会给和工具处于同一设备的Lark推送调试消息，后续流程类似移动端。

注：后续流程讲解基于移动端。

当调试成功建立后会工具上会弹出“连接已建立”提示，你可进行结束调试操作，或者切换到 PC 调试模式。
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6ed1b40d933e7cd79e55baf1a6cf68a1_1CiKzHI1Xy.png)
#### 远程调试窗口
当调试建立后，会有对应的远程调试窗口被打开，远程调试窗口目前可以进行元素（Elements）和  js 代码相关的调试。关闭远程调试窗口会结束调试进程。
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7231ba9d05d0c0a0edf9aa8ab7a7974d_nhJXVnDgjD.png)
#### Elements 面板
你可审查元素，改变元素属性或改变样式并实时查看效果。
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/137b608ef6cb2e88d27b54c6e125d591_StAu3Erwl4.png)
#### Console 面板
在远程调试的调试器里，开发者可以在 Console 面板里对代码进行调试，在 Sources 面板里查看小程序的源代码并进行断点单步调试。
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d3e45133556faa6ea702fa9f96c83bfd_WSkLzlFqGj.png)
#### Sources 面板
如果你希望在 Sources 面板查看源代码，请查看file协议下,__debugDist__/splits文件夹下的文件。
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/574f79fbca3beb5b756ad9cfc23c3b81_D6ZjWeYkE2.png)

除了可以在调试器进行单步调试，开发者还能在代码中手动插入 debugger; 语句进行断点调试。因此，如果想要在小程序启动的尽早时刻断点，可以在进入远程调试之前，手动在需要断点处的代码插入 debugger; 语句来实现。
Elements 面板的操作和开发者工具调试模拟器时的操作一致。
### 真机表现
调试过程中的手机端展示如下所示。
>当手机无网络或者进入了断点状态时，将会出现一个浮层提示并阻止进一步的操作。
>
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e9d7d8f0f0f38f2529e704ec0e29af68_zWxqcPXVBk.png)

>正常运行状态如下

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c41e2b7213528caa5ad1d6a3284eea0e_LyYdMfMJli.png)

>你也可以在这里结束调试，点击结束后工具那边也会同步结束调试。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2b94955b00a4f20b12e781bd7a59fd75_hpKoikvXS3.png)

### 真机API兼容说明
目前端上有些API不支持，如果开发者在调试过程中遇到异常，可先查看API是否支持，然后选择使用预览模式对不支持API进行测试。详见[真机调试API兼容性说明](/document/uYjL24iN/uEzMzUjLxMzM14SMzMTN/remote-debug-api-available)。

