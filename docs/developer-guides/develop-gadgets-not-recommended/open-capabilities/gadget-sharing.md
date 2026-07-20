---
document_id: '6965379541105082373'
directory_id: '6907567266540371970'
title: 小程序分享
full_path: /uYjL24iN/uYzM4IjL2MDOy4iNzgjM
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Open Capabilities
- Gadget Sharing
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:50Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYzM4IjL2MDOy4iNzgjM
---

# 小程序分享

本指南详细介绍如何将小程序分享到Lark中。小程序分享功能可以让你的应用具有被分享的能力。你的小程序可以被分享到群或者某个人，被分享的用户可以点击卡片打开并使用你的小程序。

::: note
移动端：Lark2.9 及以上版本支持 ；PC 端：Lark3.11 及以上版本支持
:::

## 实现分享相关的生命周期函数

实现页面的生命周期函数 onShareAppMessage，指定分享的标题、跳转链接和预览图，最终以卡片形式分享到Lark群或个人。默认使用当前屏幕截图的上半部分作为预览图，你可以通过 imageUrl参数设置自定义预览图。

::: note
关于页面生命周期更多详细信息，请查看[启动页面](/document/uYjL24iN/uQDNuQDNuQDN)。
:::

```js
Page({
    onShareAppMessage: function (opt) {
        console.log(opt);
        return {
            title: opt.from === 'button' ? 'Button Share' : 'Menu Share',
            path: '/page/API/pages/share/share?a=b&from=' + opt.from,
            PCPath: '/page/API/pages/share/share?a=b&from=' + opt.from,
            PCMode: 'sidebar-semi',
            imageUrl: "https://xxx.jpg",
            success(res) {
                console.log('success', res);
            },
            fail(errr) {
                console.error(errr);
            },
        };
    }
})
```
::: note
当不需要自定义预览图时，无需传入imageUrl参数。
更多关于小程序分享功能的参数说明，请查看[自定义分享内容](/document/uYjL24iN/uQDNuQDNuQDN)。
:::

## 配置分享功能入口

用户可以从小程序菜单分享页面，也可以从应用内通过按钮分享页面。

### 从菜单分享小程序页面

完成对页面的分享功能配置后，用户进入该页面便可在小程序菜单列表看到分享按钮。

用户点击 ***分享*** 按钮，并选择分享给Lark群或个人，最终以消息卡片的形式展现。


![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/2f0317816e7d6f22e898a139c8a49dd8_image.png)

用户点击卡片，会跳转到小程序对应的页面。


![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/f44179adbbceda229c62b8ada4b52dd8_image.png)

### 从应用内部分享小程序页面

你也可以在页面中，增加 [button](/document/uYjL24iN/uIjNuIjNuIjN)，并将 open-type 设置为 share，以实现在应用内部分享小程序。

用户点击分享按钮，并选择分享给Lark群或个人，最终以消息卡片的形式展现。


![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/836495e890e1565dc46b5a103edb72a1_image.png)
