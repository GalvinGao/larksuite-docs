---
document_id: '7115363412721205254'
directory_id: '7021842990277787653'
title: 应用快捷入口
full_path: /uAjLw4CM/uYjL24iN/extensions/app-shortcut
breadcrumb:
- Developer Guides
- Configure App Entry
- APP shortcut
document_type: GuideDocumentType
updated_at: 2022-07-04T09:37:27Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/extensions/app-shortcut
---

# 应用快捷入口
## 功能亮点

搜索更快捷：搜索应用名称，可以快速进入常用快捷入口

功能更简单：支持快捷入口名称召回相关应用，找信息更简单


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/46ea206286b06f5c4d0411d76b0da153_36jnflxZPh.png?lazyload=true&width=1640&height=426)

## 配置流程


1. 进入 [Lark开放平台](https://open.larksuite.com/app)/应用详情页/扩展，找到「应用快捷入口」模块

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/16cb8c3be43983cfe9c79917f803467c_LCL0EJB8y0.png?lazyload=true&width=1804&height=805)

2.  启用 应用快捷入口（最多可支持添加4个快捷入口）
3.  填写快捷入口的名称，并配置对应的applink地址

-    如何设置快捷入口名称：入口展示文案最好大于等于四个字，且表意尽量清晰。需要考虑到用户在此时还未进入应用场景，过于简洁的名称可能无法清晰传达功能作用。例如：对于差旅报销应用，“机票预订”表意就比“机票”更清晰

-   如何配置APPlink：快捷入口能够帮普通用户快速找到复杂应用里面常用功能，因此配置的APPlink必须属于当前应用，即通过APPlink协议拼接地址时需要保证APPID是一致的。
	- 网页应用的APPlink协议可参考[打开网页应用](/document/uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-an-h5-app)
	- 小程序应用的APPlink协议可参考 [打开小程序](/document/uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-a-gadget)


4.  填写完成后，按照应用更新流程进行应用发布即可生效，提交后搜索5分钟内就会生效

## FAQ

-  Q: 搜索快捷入口对Lark版本有什么要求？
-
A: 需要将Lark升级至4.9版本及以上，可以看到配置后的应用二级入口。仅支持电脑端，移动端暂时不支持快捷入口的展示

-  Q: 搜索哪些词可以召回应用？支持模糊匹配吗？

A: 只支持搜索快捷入口的**完整全称**召回此应用，暂时不支持模糊匹配哦～

-  Q: 应用能被召回的逻辑是什么呢？

A:快捷入口的名称会被当作应用的“关键词”一起召回。
