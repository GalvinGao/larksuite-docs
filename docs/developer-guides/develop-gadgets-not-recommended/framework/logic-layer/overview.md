---
document_id: '6965379543683629062'
directory_id: '6907567266536996865'
title: 概述
full_path: /uYjL24iN/uEDOzUjLxgzM14SM4MTN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- Logic Layer
- Overview
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:14Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEDOzUjLxgzM14SM4MTN
---

# 概述
开发者上传的 JavaScript 代码会合并成一个文件，逻辑层就是指将这部分代码运行在定制的 JavaScript 引擎中，由引擎提供小程序的运行环境，同时引擎通过调用宿主（Lark）的能力，为小程序提供与宿主交互的功能。

逻辑层的功能如下：
1. 提供 JavaScript 的原生能力
2. 提供 App 对象用于小程序注册
3. 提供 Page 对象用于页面注册
4. 增加 getApp 和 getCurrentPages 方法，分别用来获取 App 实例和当前页面栈
5. 提供丰富的API来增强小程序的能力
6. 提供模块化能力，每个页面有独立的作用域
> 小程序框架的逻辑层并非运行在浏览器中，因此 JavaScript 在 web 中一些能力都无法使用，如 window，document 等
