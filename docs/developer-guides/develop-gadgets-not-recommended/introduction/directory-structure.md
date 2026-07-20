---
document_id: '6965379541104066565'
directory_id: '6907567269107695618'
title: 小程序目录结构
full_path: /uYjL24iN/ukjMukjMukjM
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Introduction
- Directory Structure
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:08Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjMukjMukjM
---

 # 目录结构
小程序包含一个描述整体程序的 app 和多个描述各自页面的 page。
目录结构大致如下：

```

├── app.js
├── app.json
├── app.ttss
├── project.config.json
└── pages
```

以上目录结构中的 pages/可以根据实际情况进行灵活配置。
其中一个小程序根目录包含下面三个文件，如下：
|文件|必须|作用|
|-----|---|-----|
|app.js|是|小程序入口逻辑|
|app.json|是|小程序公共设置，例如：所有页面路径等|
|app.ttss|是|小程序公共样式|

同时，小程序每个页面也有对应的目录结构。具体内容可以参考以下示例：
```
├── pages
│   │── home
│   │   ├── home.ttml
│   │   ├── home.js
│   │   ├── home.json
│   │   └── home.ttss
│   └── user
│       ├── user.ttml
│       └── user.js
├── app.js
├── app.json
├── app.ttss
└── project.config.json
```
每个小程序的页面都可以由以下四个文件组成：

|文件|必填|作用|
|-----|---|-----|
|*.js|是|页面逻辑|
|*.json|否|页面配置|
|*.ttss|否|页面样式表|
|*.ttml|是|页面结构|

**注意**：为了方便开发者减少配置项，描述页面的四个文件必须具有相同的路径与文件名。

## 上传的文件
在项目目录中，\*.js、\*.json、\*.ttml、\*.ttss等文件会被上传到服务器进行编译（其中 ttml 和 ttss 文件仅针对在 app.json 中配置了的页面）。这些文件上传后经过编译生成其他文件，因此上传之后无法直接访问到。

