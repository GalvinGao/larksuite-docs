---
document_id: '6967331173081939973'
directory_id: '6907567269107531778'
title: 云文档接口快速入门
full_path: /ukTMukTMukTM/uczNzUjL3czM14yN3MTN
breadcrumb:
- Server API
- Docs
- Docs Overview and Best Practice
document_type: GuideDocumentType
updated_at: 2021-11-17T10:28:18Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN
---

# 云文档接口快速入门
:::note
通过这篇文档，你可以了解云文档 API 的能力，以及一些最佳实践场景，帮助你快速构建属于自己的想法。
:::

## 云文档接口介绍
云文档是文档、表格、空间这一系列创作工具的统称。通过调用云文档 API，你可以完成一系列的自动化、批量化操作。云文档 API 由以下几个模块组成：
:::html
<md-tag mode="inline" type="">文件管理</md-tag>：由文件、文件夹、权限、评论等模块组成，可以完成对应的操作
:::
- ==文件==：上传、下载、复制云空间中的文件
- ==文件夹==：创建、管理云空间中的文件夹
- ==权限==：管理云空间中的文件权限
- ==评论==：获取、添加、编辑文档中的评论

:::html
<md-tag mode="inline" type="">文档</md-tag>：用于创建、获取、编辑在线文档<br>
<md-tag mode="inline" type="">电子表格</md-tag>：用于获取、编辑在线表格<br>
<md-tag mode="inline" type="">多维表格</md-tag>：用于获取、编辑多维表格
:::

## 权限说明
### 应用权限
在使用云文档的权限之前，请首先确保你的应用已经申请了对应的权限。应用权限可以在开发者后台进入应用详情页后进行管理，详情可以参考：[应用权限](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN)
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/822b6cfbde7b6bbe1f4dcca7098eb6e1_nWjujY92R9.png)

<br>
### 调用身份
:::note
不论使用何种 token 进行调用，都必须确保这个用户或者应用具备文档的权限，否则会调用失败。
:::
:::html
云文档大部分接口都支持通过 <md-tag mode="inline" type="token-user">user_access_token</md-tag> 和 <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag> 两种身份进行调用，但仍然有部分接口只支持其中一种身份，请开发者在调用前，仔细阅读对应接口的说明文档。<br>
:::
以四个主要场景来说明调用身份的具体使用方法：（以 [编辑文档内容](/document/ukTMukTMukTM/uYDM2YjL2AjN24iNwYjN) 为例）<br>
| 具体场景及调用身份 | 实现方法 | 
| --------- | --------- | --------- |
| 以用户身份编辑自己的文档<br>( user_access_token )| 1）获取自己的 user_access_token <br> 2）获取要编辑的文档token（方法见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction)）<br> 3）通过[编辑文档内容](/document/ukTMukTMukTM/uYDM2YjL2AjN24iNwYjN) 接口调用编辑 |
| 以用户身份编辑他人的文档<br>( user_access_token ) | 1）获取他人的 user_access_token <br> 2）获取要编辑的文档token（方法见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction)）<br> 3）通过[编辑文档内容](/document/ukTMukTMukTM/uYDM2YjL2AjN24iNwYjN) 接口调用编辑 |
| 以应用身份编辑自己的文档<br> ( tenant_access_token ) | 1）获取应用的 tenant_access_token<br> 2）获取要编辑的文档token（方法见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction)）<br> 3）通过[编辑文档内容](/document/ukTMukTMukTM/uYDM2YjL2AjN24iNwYjN) 接口调用编辑 |
| 以应用身份编辑他人的文档<br> ( tenant_access_token )|1）获取应用的 tenant_access_token<br> 2）他人的文档授权应用<br> 3）获取要编辑的文档token（方法见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction)）<br> 4）通过[编辑文档内容](/document/ukTMukTMukTM/uYDM2YjL2AjN24iNwYjN) 接口调用编辑 |
<br>

## 请求返回值
返回值结构如下，如果报错，请查询[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
```json
{
    "code": 200,
    "msg": "sample err msg",
    "data": {}
}
```

