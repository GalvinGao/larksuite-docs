---
document_id: '7069329034605658117'
directory_id: '6907567269107613698'
title: 小程序搬家工具（不推荐）
full_path: /uYjL24iN/uEzMzUjLxMzM14SMzMTN/tool-of-transforming-wechat-miniprogram
breadcrumb:
- Developer Guides
- Tools and SDKs
- Development Tools
- Gadget migration tool (Not Recommended)
document_type: GuideDocumentType
updated_at: 2024-12-05T10:46:10Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEzMzUjLxMzM14SMzMTN/tool-of-transforming-wechat-miniprogram
---

#  小程序搬家工具

# 什么是搬家工具

若你之前开发过微信/钉钉小程序，可以通过我们的搬家工具来快速将微信小程序/钉钉小程序的部分代码迁移到Lark小程序上，从而降低开发成本。

搬家工具可进行静态语法上的转换，将微信/钉钉小程序的常用接口转化为能够被Lark识别的方法，从而在Lark中正常运行。

需要注意的是：工具做不到运行时差异的抹平，也做不到一个 API 从无到有的过程。所以，需要大家根据转换过程中打出的日志，再对部分功能进行适配。

# 如何使用搬家工具

## 打开搬家工具并选择待转化的应用类型

-   在IDE的右上角入口点击【搬家工具】
-   选择转化类型——微信小程序转化 或 钉钉小程序转化

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2533fe074e6de3d048a5fd27fe9bd90b_43PmrAKOys.png)

## 转化为Lark小程序并进行调试
- 输入「待转化的项目路径」和「转化后的Lark小程序目录」，点击“确定”就会开始转化
:::note
若选择了微信小程序转化，需要注待转化的微信小程序中需要有project.config.json文件，具体可参考[微信小程序:工具配置](https://www.cnblogs.com/lizm166/p/9547816.html)。且待转化的微信小程序需要有dist文件。若你的项目中尚不包含此内容，先用微信开发者工具编译此项目
:::

- 转化成功后即可打开Lark小程序项目，你可以继续调试和开发，也可以直接使用新的Lark小程序包

    -   若原小程序中有Lark不支持的组件或接口，则搬家工具会给到转化失败的提示，需要你二次确认并根据需求进行适配

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/72abb486acd5c5081a654c03f6d1228b_pGZhBg9pXK.png)

-   点击「打开」可以发现代码中的相关内容已经转化成功，且项目可以正常编辑与调试

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/99a74b8f2541a45b81a24ad7266d1860_cF7lkN7yC4.png)

## FAQ

Q：我的微信/钉钉小程序借助了第三方框架来做开发，请问应该怎么转化

A：（1）如果你是通过taro或uniapp开发的微信/钉钉小程序，则你可以通过[taro 框架](/document/uYjL24iN/uEzMzUjLxMzM14SMzMTN/develop-gadget-with-taro) 或 [uniapp 框架](/document/uYjL24iN/uEzMzUjLxMzM14SMzMTN/develop-gadget-with-uni-app)直接开发Lark小程序。（2）如果你使用了其他框架，则可以先用npm功能导入框架，并在编译完成后再使用搬家工具。

# 附录 搬家工具帮你完成了哪些工作

:::note
以下以微信小程序转化工具为例
:::

针对 xml 文件

-   路径后缀转换成ttml
-   提示Lark小程序不支持的内置组件
-   Import 标签后缀路径转成ttml
-   Wxs 标签转换成 sjs

针对 wxss 文件

-   路径后缀转换成ttss
-   @import 引入路径后缀转成ttss

针对 js 文件

-   Wx API 转成 tt API
-   不支持的 API 提示

针对 wxs 文件

-   路径后缀转换成 sjs

针对 json 文件

-   app.json usingComponents挂载到各个页面的json文件中
-   app.json useExtendLib 字段不支持提示

