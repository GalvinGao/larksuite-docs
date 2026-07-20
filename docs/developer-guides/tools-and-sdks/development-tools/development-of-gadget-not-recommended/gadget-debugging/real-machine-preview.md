---
document_id: '7034327431302873094'
directory_id: '7027037607365754885'
title: 真机预览
full_path: /uYjL24iN/uEzMzUjLxMzM14SMzMTN/feishu-developer-tools-real-machine-preview
breadcrumb:
- Developer Guides
- Tools and SDKs
- Development Tools
- Development of Gadget (Not Recommended)
- Gadget Debugging
- Real Machine Preview
document_type: GuideDocumentType
updated_at: 2022-11-17T05:56:30Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEzMzUjLxMzM14SMzMTN/feishu-developer-tools-real-machine-preview
---

# Lark开发者工具-真机预览

真机实时预览功能支持监听代码的修改，当代码发生修改并保存后，开发者工具会实时编译并将编译后的小程序推送给Lark移动端。**可以实现编写小程序时快速预览，免去了每次查看小程序效果时都要扫码的麻烦。**

-   从工具栏点击「预览」，然后选择移动端或PC端，扫码即可预览。
-   小程序预览依赖以下条件：
    -   真机实时预览功能依赖局域网通信，需要保证当前设备和手机在同一网络下
	-   需要移动端Lark版本 >= 3.12.0
	-   从左上角的登录组件指令或者模拟器界面已经完成Lark账号登录；
	-   在应用项目中的配置project.config.json填写了正确的 appId，appId 的获取参考[文档](/document/home/develop-a-gadget-in-5-minutes/create-a-custom-app)

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8d597407958dc45696fcd29c8d20fe3d_tkIb58fJyj.png)
