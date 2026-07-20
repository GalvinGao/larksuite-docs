---
document_id: '7148715232021807110'
directory_id: '7268257303224172549'
title: API 调试台使用指南
full_path: /tools-and-resources/api-explorer-guide
breadcrumb:
- Developer Guides
- Tools and SDKs
- API Explorer Guide
document_type: GuideDocumentType
updated_at: 2024-02-08T06:37:32Z
source_url: https://open.larksuite.com/document/tools-and-resources/api-explorer-guide
---

# API 调试台使用指南
# 一、工具概览

API 调试台是一款一站式的、能提升接口调试效率的Lark应用开发工具。
通过 API 调试台，你无需编写代码，即可快捷完成服务端的接口调用。此外，我们提供了以下几点功能，提升你的接口调试效率：
- **自动获取鉴权凭证**：绑定你的应用一键获取 token，每次调试无需再额外发起请求获取
- **内置应用权限申请**：调通接口所需权限一目了然，无需跳转后台，调试台内快捷申请
- **接口调试** **示例代码**：提供多语言示例代码，点击复制即可快速复用至业务代码中



# 二、工具入口

在Lark开放平台的 API 开发文档界面，点击“尝试一下”即可进入该接口的 API 调试台界面。你也可以收藏此链接，[点击跳转立即体验 API 调试台](https://open.larksuite.com/api-explorer?from=guide)。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/056f8d7849911d312244d6f9ed15651a_9DRIE3fvY6.png?height=432&lazyload=true&maxWidth=600&width=1692)


# 三、功能介绍

## 绑定应用身份

首次使用 API 调试台，需要你先登录。登录后，你需要绑定一个你的应用身份，再进行调试。（如你名下没有应用，则需要先创建应用再进行调试）

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/afdc92087ad164a791f416d6c4bdb1bc_i1exftnoIx.png?height=822&lazyload=true&maxWidth=600&width=1948)

绑定后你也可以随时切换其他的应用，无需担心。通过测试企业功能，开发者创建的应用分为**正式版**和**测试版。** API 调试台支持这两种应用的调试。

:::note
注意：调试正式版将会直接影响线上数据，我们推荐开发者[在开发者后台绑定测试应用](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)，在开发阶段使用测试版本进行调试。
:::

## 一键获取鉴权凭证

要调用服务端 API，应用需要从Lark开放平台获取相应的访问凭证(access token)；访问凭证代表应用从平台、租户（指公司或者团队）、用户手中获得的授权。
- **tenant_access_token:** 租户授权凭证，使用该access token，应用将代表公司或者团队执行对应的操作，比如获取一个通讯录用户的信息。[获取租户访问凭证](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/g#top_anchor)
- **user_access_token:** 用户授权凭证，使用该access token，应用将代表用户执行对应的操作，比如通过 API 创建一篇云文档或者一个日程。[获取用户访问凭证](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/get-)

使用访问凭证时，需要在调用接口时将其填入到 HTTP 请求的请求头参数中。每个 Open API 的接口中都声明了该接口需要的访问凭证类型和获取方式。

当你使用 API 调试台，则无需额外调用「获取租户/用户访问凭证」接口，一键点击即可获取接口所需鉴权凭证，自动填充在接口请求参数中。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/08d4257149addefcb4814bd6fc12efdb_zhfOV8qmKA.png?height=816&lazyload=true&maxWidth=600&width=2080)


:::note
注意：访问凭证有有效期限（7200s），凭证过期后需要刷新
:::

## 发起接口调试

### 填写参数

API 调试台中，你可以在表单中填写请求头、请求体和路径参数，修改各入参的默认值发起实际的调用，来体验不同入参情况下 API 的响应。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a5675bcaca532c1785b90b424a0335a8_lf2SKuxxAJ.png?height=914&lazyload=true&maxWidth=600&width=2846)


### 配置权限

权限和**接口**能力、数据**字段**、订阅**事件**绑定。如你正在调用的接口需要的权限有缺失，API 调试台界面会有小红点报错提示，需要你及时发起申请。[应用权限概述](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN)

本租户的应用管理员可以自定义租户内哪些权限需要审核，哪些权限可以免审直接生效。权限类型包括：
- **免审权限**：申请后立即生效的权限，这类权限不需要管理员审核。
- **需审核权限**：申请后，需要创建应用版本并提交审核，由管理员审核通过才可生效。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fe10e4de35b63e9970e466c3237c62cc_oJxU7Rnq7Q.png?height=808&lazyload=true&maxWidth=600&width=2842)


### 发起调试

完成参数配置后，点击「开始调试」，即可查看调试结果。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6e2fa0becacbf1837738e22d9704200d_1DNro0zsLL.png?height=1024&lazyload=true&maxWidth=600&width=2708)

如调试失败，你可以查阅 API 文档自助排查问题，或复制 log ID 后联系技术支持进行问题排查。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b3009db19f5a0304dc85d9291bcf3680_dZ2YW8r2Ox.png?height=1042&lazyload=true&maxWidth=600&width=2702)


## 查看调用历史

按照应用身份维度，API 调试台存储展示了你最近 30 天的调用历史。如果你需要调用一批相关接口，可以通过查阅调用历史、复用历史调用参数，也可以使用之前的调用参数来重新发起调试。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cd745be314ac8f73fedd29c0c8634606_F2n8uCrCN5.png?height=1144&lazyload=true&maxWidth=600&width=1760)


## 复用示例代码

在编辑完请求参数后，API 调试台会根据你输入的请求参数实时生成多语言调用示例，开发者可以直接一键拷贝示例代码，复用到自己的业务代码中。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cfe545948dfc24934e2863ba36560c41_wf5hWQEQmj.png?height=922&lazyload=true&maxWidth=600&width=2702)

# 四、反馈建议

如你在调试过程中，对 API 接口或工具本身有任何产品建议和反馈，可以通过 API 调试台的反馈界面，随时向我们反馈你的使用体验。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ec6700269eb02f76529070e6a40c2d0d_hVq3AuVNWP.png?height=1134&lazyload=true&maxWidth=600&width=1822)
