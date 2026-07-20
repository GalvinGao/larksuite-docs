---
document_id: '7270779605451522054'
directory_id: '7270719284443545605'
title: 快速上手
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/03-cloud-document-widget-quick-development-guide/03-cloud-document-widget-quick-developme
breadcrumb:
- Developer Guides
- Develop Docs Add-ons
- Overview
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:24Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/03-cloud-document-widget-quick-development-guide/03-cloud-document-widget-quick-developme
---

# 快速上手

## 创建小组件

### 加入小组件开发技术支持群

![fbc8b792-8cf8-4015-8550-2713a8bec275.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3354dd05848d5e3a1f697e8dc9b37cf9_bcwBYb7O6J.png?height=1280&lazyload=true&width=1180)

### 安装开发者工具

>- 如您安装过开发者工具，请先执行`npm uninstall @bdeefe/opdev-cli -g`清理环境，完成之后再使用安装包安装，避免路径被覆盖。
> - 使用命令行安装需要依赖 Node.js。安装方式请参考 [Node.js](https://nodejs.org/)，安装 Node.js 的同时会自动安装 npm。
> - opdev 更多指令操作见：[Lark开发者工具-命令行 - 开发文档 - Lark开放平台](/document/no_class/feishu-developer-tool---command-line#59e404d7)
```js
npm install @lark-opdev/cli@latest -g

npm uninstall @bdeefe/opdev-cli -g

# 验证是否安装成功

opdev help
```

### 登录账号

1. 安装成功后，执行以下指令，在打开的登录页面按照提示进行登录：
```
$ opdev login
```
2. 选择开发环境，这里选择Lark开发环境
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/16e8c8eb41aa74233742d45d71d95eab_oaJzW8aiKh.png?height=320&lazyload=true&width=1952)
3. 选择开发环境后，会自动启动浏览器跳转到登录页面进行登录，登录成功后，再次执行`opdev login`将看到如下输出：
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/62266c6f30a418b379f46881ed44bfc6_2uEmen8Lcb.png?height=350&lazyload=true&width=2634)

### 创建项目

在命令行终端执行以下指令创建示例项目：
```
$ opdev create [FolderName]
// 示例
$ opdev create demo
```

### 选择组件类型

此处选择docs-addon
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0d738c3fb4b3a68ad5d608598615c3ef_1p9aL0ovie.png?height=244&lazyload=true&width=1686)

### 生成云文档小组件

输入Enter键，此时会自动创建云文档小组件，可点击控制台的链接访问开发者控制台来查看系统创建的云文档小组件相关信息。

**备注说明：不支持在Lark测试企业中创建和调试**

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2b97c64dedadb99b08d90fed2279ae09_6xy7KbtTqz.png?height=166&lazyload=true&width=1280)

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ec90ac7b60c61c55b9c7815c86054689_25jsnsgrIv.png?height=1506&lazyload=true&width=1966)

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0342e300fe38bc7579220d0d3a16a67f_gJsDZu1pgX.png?height=1490&lazyload=true&width=1796)


### 示例小组件介绍

示例工程是一个文本绘图小组件，可以在文档中插入 Mermaid绘图工具，用几行文本（代码）快速创建流程图、时序图等复杂图形。
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/09edcdcb1ffd286fdf25f9ea8b843429_sa6jzhZ4zy.png?height=684&lazyload=true&width=1770)

```
├── README.md
├── app.json  #开发者配置文件
├── node_modules
├── package.json
├── src #源代码目录
├── tsconfig.json #ts配置
└── webpack.config.js #构建工具
```
## 调试小组件

进入项目根目录，安装依赖，启动项目：
```
cd demo
// 也可以选择其它包管理器（pnpm、yarn 等）
npm i
npm start
```
在打开的测试页面中可以通过菜单插入示例小组件（本地小组件名称为 blockTypeID）
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/202fb8dbcdb87acf6cb25a53f10afe47_kD5YfrXhLV.png?height=1354&lazyload=true&width=2204)
## 上传小组件

在命令行终端中执行下面命令，按提示操作即可完成小应用的发布。
```
npm run upload
```
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8eea5eeed36ebc7e411262fb01456c57_AXPdvHBLvw.png?height=510&lazyload=true&width=1960)
## 发布小组件

1. ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5040277c5ab67c41245029e52bfdc142_qypGPbVTpj.png?height=1624&lazyload=true&width=4536)
2. 进入开发者后台，选择刚才说上传的程序包，填写小组件基础信息，并且保存
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/58831e0394a9a06a9cc08f4746fc4ace_Srmab3Zegd.png?height=1528&lazyload=true&width=2384)
3. 随后在应用发布一栏，创建一个版本发布
创建版本完成，申请线上发布，随后由后台管理员进行审核，审核通过后就发布完成了。
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e4c0957f7cbc3aa7e52b4a65477a3531_UG6h9jnT43.png?height=1508&lazyload=true&width=2990)
## 小组件国际化配置

小组件国际化配置可以让您的小组件发布之后，在不同的语言环境中展示对应的小组件名称和介绍。
如需启用国际化配置，请先至开放平台 - 凭证与基础信息 - 国际化配置新增不同语言的名称和应用描述。
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a1e4b1892af03738f24fc3c92cb12733_LOxMh2qaWJ.png?height=2012&lazyload=true&width=2842)
随后，在您的小组件内添加小组件在不同语言下的基础信息配置并发布，便可以完成小组件国际化配置。
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b812ba08aab9fa16aac48cddbf18bf1b_VxXDwh9bgc.png?height=1838&lazyload=true&width=2312)
小应用内国际化可以结合您的技术选型，选择合适的方案（如[react-i18next](https://react.i18next.com/)）

我们提供了对应的接口可以获取当前用户的语言：[Env.Language.getLanguage](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Env.Language.getLanguage)

## 小组件配置说明

### 项目配置

项目配置位于项目根目录下的`app.json`中，主要包含以下配置：
| 字段名             | 类型                              | 说明                            | 示例                                                       |
| --------------- | ------------------------------- | ----------------------------- | -------------------------------------------------------- |
| manifestVersion | number                          | Manifest 版本                   | 1                                                        |
| appID           | string                          | 应用 ID，从Lark开放平台获取               | cli_a33d20ce6eb8****                                     |
| blockTypeID     | string                          | blockTypeID，在Lark开放平台开启小组件能力后获取 | blk_63b79cd3cf80000150d6****                            |
| contributes     | Record<string, ContributeInfo> | 组件类型&附属视图配置                   | {"addPanel": {"initialHeight": 46,"view": "index.html"}} |
| projectName     | string                          | 小组件名                          | Docsaddon-Demo                                           |
更多配置请参考
[组件配置](/document/uAjLw4CM/uYjL24iN/docs-add-on/03-cloud-doc-block-rapid-development-guide/appjson-configuration-instructions)。


## 安全配置（必选，否则会影响小组件正常使用）
参考 [安全配置说明](/document/uAjLw4CM/uYjL24iN/docs-add-on/08-cloud-doc-block-security-configuration/cloud-doc-block-security-configuration)

## 开源示例代码参考
[阅读数据](https://github.com/docsaddon/page-viewers)
