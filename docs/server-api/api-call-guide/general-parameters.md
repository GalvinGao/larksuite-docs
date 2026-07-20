---
document_id: '6995834315541037062'
directory_id: '7262910572679708677'
title: 通用参数
full_path: /ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/terminology
breadcrumb:
- Server API
- API Call Guide
- General parameters
document_type: GuideDocumentType
updated_at: 2023-11-10T07:54:14Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/terminology
---

# 通用参数
为了满足多样化的应用场景，Lark开放平台提供了丰富的 API 和事件，供开发者调用或订阅。这些 API 和事件会涉及到一些通用参数，在此进行统一介绍。
通用参数可能作为请求参数，也可能作为响应参数。在使用Lark开放平台提供的 API 和事件之前，建议先了解一下这些通用参数的含义。
## 应用相关

### app_id

`app_id` 是Lark开放平台应用的唯一标识。在创建应用时，由系统自动生成，用户不能自行修改。可以在[开发者后台](https://open.larksuite.com/app)的 **凭证与基础信息** 页面查看 `app_id`。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/75a323a2311bf02b984645398d9a04f4_IzV8lIb35K.png?height=524&lazyload=true&maxWidth=600&width=3594)

### app_secret

`app_secret` 是应用的秘钥。在创建应用时，由系统自动生成，在调用某些 API 时，需要将 `app_secret` 作为请求参数之一。用户可以在[开发者后台](https://open.larksuite.com/app)的 **凭证与基础信息** 页面查询或重置秘钥。

### app_ticket

`app_ticket` 是商店应用使用到的一个字段，用于提升商店应用的安全性。
调用[接口](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_access_token)获取商店应用的 `app_access_token` 时，需要携带 `app_ticket` 作为请求参数之一。
创建商店应用并配置事件的请求地址后，Lark开放平台会以 1 次/小时的频率向应用推送 `app_ticket`，用户也可以调用[接口](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_ticket_resend)，主动触发 `app_ticket` 的推送。


## 企业相关

### tenant_key

`tenant_key` 是租户唯一标识，在实际应用场景中，对应一个企业。应用有如下两种方式获取 `tenant_key`：
- 企业安装应用时，开放平台通过事件推送给应用。
- 用户登录授权时，Lark开放平台会返回 `tenant_key`。

### department_id

自定义的部门 ID， 如果没有自定义部门 ID，则开放平台会随机生成一个字符串作为自定义部门 ID。
:::note
2021 年 4 月起，开放平台更新了部门管理相关接口策略，支持复用已经删除部门的 `department_id`。更新后，`department_id` 不再是全局唯一，不推荐作为应用中部门的唯一标识，建议使用 `open_department_id` 替代。
:::

### open_department_id

开放平台自动生成的部门 ID，以 `od-` 开头。`open_department_id` 全局唯一，即跨应用、跨开发主体的 `open_department_id` 都是不同的。

## 用户相关

Lark对用户身份体系有充分地设计，为了满足不同开发场景的需求，Lark设置了 `user_id`、`open_id` 和 `union_id` 等不同的用户标识。详细的介绍，请参考[用户相关的 ID 概念](/document/home/user-identity-introduction/introduction)。

开发一个应用时，合理的 ID 选择逻辑如下：
- 创建的应用不存在跨应用的数据关联互通场景，使用 `open_id` 即可。
- 需要跨应用关联数据，但是其开发者归属同一个企业组织，可以使用 `union_id`。
- 需要跨应用关联数据，并且这些应用可能是不同的组织开发的，但是应用的用户归属在同一个企业内，则使用 `user_id`。

## 访问凭证

为了提升 API 调用的安全性，Lark开放平台设计了访问凭证机制。在调用接口前，应用需要先获取所需的访问凭证。

访问凭证也称为 `access_token`，代表应用从平台、租户（公司或者团队）、用户手中获取的授权。访问凭证包括三种：
- 应用访问凭证，参数名为 `app_access_token`。
- 租户访问凭证，参数名为 `tenant_access_token`。
- 用户访问凭证，参数名 `user_access_token`。


了解如何获取访问凭证，请参考[获取访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)。

了解Lark开放平台的权限体系，可以参考[访问权限介绍](/document/home/introduction-to-scope-and-authorization/overview)。

### app_access_token

`app_access_token` 即应用访问凭证。开放平台可根据 `app_access_token`，识别调用方的应用身份，应用可以访问应用自身相关的信息，不归属到具体的企业或者用户。
如果要了解如何获取应用访问凭证，请参考[商店应用获取 app_access_token](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_access_token)、[自建应用获取 app_access_token](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_access_token_internal)。

### tenant_access_token

`tenant_access_token` 即租户访问凭证。使用 `tenant_access_token`，应用将代表公司或者团队执行对应的操作，比如获取一个通讯录用户的信息。
如果要了解如何获取 `tenant_access_token`，请参考[商店应用获取 tenant_access_token](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token)、[自建应用获取 tenant_access_token](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token_internal)。

### user_access_token

`user_access_token` 即用户访问凭证。使用 `user_access_token`，应用将代表用户执行对应的操作，比如用户通过 API 创建一篇云文档或者一个日程。如果要了解如何获取用户访问凭证。请参考[获取 user_access_token](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/authen/access_token)。


## 其他

### chat_id

`chat_id` 是会话（包括单聊、群聊）的唯一标识。

### open_message_id

`open_message_id` 是消息体的唯一标识。
