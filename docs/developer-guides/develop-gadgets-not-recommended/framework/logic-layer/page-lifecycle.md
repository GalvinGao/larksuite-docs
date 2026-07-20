---
document_id: '6965379543684612102'
directory_id: '6907567266536996865'
title: 页面生命周期
full_path: /uYjL24iN/ugjNzUjL4YzM14CO2MTN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- Logic Layer
- Page Lifecycle
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:22Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugjNzUjL4YzM14CO2MTN
---

# 小程序页面生命周期概述
 在小程序页面打开时会有相应的生命周期函数回调，它们包括onLoad，onShow，onReady，onHide，onUnload。小程序开发者可根据需要在不同的合适的生命周期回调函数中实现自己的页面逻辑。
 
:::html
<md-alert type="tip">
常见的页面路由场景以及对应的路由前后页面的生命周期回调函数请查阅 [路由方式](/document/uYjL24iN/uUDNuUDNuUDN) 章节。
</md-alert>
:::

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/22c11128b722d82a4a7d6f2c29c49b7f_f9cXrGWB9L.png)





# 生命周期函数
## onLoad(Object query)
页面初始化时触发。一个页面只会调用一次。 query 来源于 [tt.navigateTo](/document/uYjL24iN/uYTOz4iN5MjL2kzM) 和 [tt.redirectTo](/document/uYjL24iN/ucTOz4yN5MjL3kzM) 等接口url字段的参数部份（例如：path?key1=value1&key2=value2）。基础库会将该部份字符串内容解析为Object。
## onShow()
页面显示/切入前台时触发。
## onReady()
页面初次渲染完成时触发。 一个页面只会调用一次，代表页面已经准备妥当，可以和视图层进行交互。 对界面的设置，如 [tt.setNavigationBarTitle](/document/uYjL24iN/uATNy4CM1IjLwUjM) 请在 onReady 之后设置。
## onHide()
页面隐藏/切入后台时触发。 如 [tt.navigateTo](/document/uYjL24iN/uYTOz4iN5MjL2kzM) 到其他页面或底部 tab 切换等。
## onUnload()
页面卸载时触发。 如 [tt.redirectTo](/document/uYjL24iN/ucTOz4yN5MjL3kzM) 或 [tt.navigateBack](/document/uYjL24iN/uADM04CMwQjLwADN) 到其他页面等。
