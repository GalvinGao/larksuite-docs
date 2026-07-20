---
document_id: '7026663896463785989'
directory_id: '7260031291679473669'
title: 应用类型简介
full_path: /home/app-types-introduction/overview
breadcrumb:
- Developer Guides
- Platform Introduction
- Application types
document_type: GuideDocumentType
updated_at: 2024-07-19T02:17:35Z
source_url: https://open.larksuite.com/document/home/app-types-introduction/overview
---

# 应用类型简介
## 什么是应用？

应用是开发者向内部或外部用户提供服务的载体，通常表现为机器人、网页、小程序等形态，也是开发者调用Lark所提供的开放能力的载体。

开发者在入驻Lark开放平台后，需要通过创建应用和开通相关权限的方式获取各种能力，通过接口调用或嵌入 SDK 的方式来进行定制开发，接入丰富的Lark开放能力，根据实际需要开发个性化的办公应用，打造高效的办公方式。



## 应用类型
根据 **应用的上架方式**，应用可分为以下两种类型：
* **企业自建应用**：由企业内部人员或企业授权的开发人员进行开发，只能在同一企业内发布和使用。
* **应用商店应用**：由第三方服务商开发，在[Lark应用中心](https://app.larksuite.com/)发布，所有Lark租户均可安装和使用。

::: note
* 如果你开发的应用只会给**同一租户内的用户使用，不需要分享到租户外部**，可选择创建企业自建应用。
* 如果希望将你开发的**应用上架到应用中心，并开放给不同的Lark租户使用**，可选择创建应用商店应用。

::: 

| **应用类型** | **开发人员** | **使用人员** | **支持的应用能力** | **是否支持上架到应用中心** |
| --- | --- | --- | --- | --- |
| 企业自建应用 | 企业内部开发者或授权的服务商开发者 | 企业内部人员 | 小程序、网页、机器人、小组件 | 否 |
| 应用商店应用 | 独立服务商 | 购买开通该商店应用的企业内部人员 | 小程序、网页、机器人、小组件 | 是，需要满足上架要求，上架流程请参考[商店应用上架流程](/document/uMzNwEjLzcDMx4yM3ATM/ugzNwEjL4cDMx4CO3ATM#5c6489f5)。 |

## **自建应用开发流程**
企业自建应用是由企业内部人员或企业授权的开发人员进行开发，在**企业内发布**上线并供内部人员使用的应用。Lark上的企业或组织可以基于Lark套件的开放能力，自主开发自建应用，以满足办公场景中的个性化需求。该类应用无需Lark团队审核，由企业租户管理员审核通过后即可使用。

::: note
* 自建应用限制在单个租户内开发、审核和使用，应用的开发者、管理员和使用者都归属在一个租户内，对外部不可见。任何一个自建应用都不可以被其他组织使用。
* Lark官方和其他第三方也不会介入到自建应用的管理流程中。
::: 

一个企业自建应用的开发上线流程如下。详细的自建应用开发流程，请参考 [开发流程概述](/document/home/introduction-to-custom-app-development/self-built-application-development-process)。

![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/d867963a8406c8820af23216e6af995c.png?height=804&lazyload=true&maxWidth=700&width=1606)


1. **加入租户**：开发者 [创建](https://www.larksuite.com/hc/zh-CN/articles/360023713853) 或 [加入](https://www.larksuite.com/hc/zh-CN/articles/360043257294)Lark租户。

3. **开发应用**：开发者在 [开发者后台](https://open.larksuite.com/app) 创建企业自建应用并完成前后端开发。

5. **发布审核**：开发者在 [开发者后台](https://open.larksuite.com/app) 提交版本发布申请，自建应用发布需要经过**租户管理员**审核，参考[自建应用审核指南](https://www.larksuite.com/hc/zh-CN/articles/360043257294)。

7. **使用应用**：应用发版通过审核后，在应用 [可用范围](/document/home/introduction-to-scope-and-authorization/availability) 内的租户内成员即可在Lark客户端的搜索、工作台、消息列表等入口查找并使用该应用。


## **商店应用开发流程**

商店应用是独立软件服务商（ISV）以Lark套件的开放能力为基础，开发并上架至[Lark应用中心](https://app.larksuite.com/)，以供Lark上的企业使用的应用。

商店应用适用于服务商研发通用的产品应用。服务商可以通过Lark平台将自己的独特能力推向千家万户，从而更容易实现一对多的传播途径。商店应用需要经过Lark官方的审核，审核通过后，方可上架至应用市场。

::: note
企业管理员和用户可以通过Lark应用中心轻松查找和安装应用程序。作为应用开发者，Lark应用中心可以帮助你获取来自国内的企业客户和收入。
:::

一个商店应用的开发上架流程如下。详细的商店应用开发流程，请参考 [流程概述](/document/uMzNwEjLzcDMx4yM3ATM/ugzNwEjL4cDMx4CO3ATM)。

![图片](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cc75b561b91a2330fe31562201563309_L8fj2lG1w9.png?height=674&lazyload=true&maxWidth=700&width=1640)

1. **入驻认证**：任何组织如果希望开发商店应用并在[Lark应用中心](https://app.larksuite.com/)上架，首先必须通过 ISV 认证，认证流程可参考[应用服务商入驻](/document/uMzNwEjLzcDMx4yM3ATM/uUzNwEjL1cDMx4SN3ATM)。

3. **开发应用**：开发者在 [开发者后台](https://open.larksuite.com/app) 创建商店应用并完成前后端开发。

5. **上架审核**：商店应用上架需要经过Lark官方运营的严格审核，参考 [开发和上架应用商店应用](/document/uMzNwEjLzcDMx4yM3ATM/uYzNwEjL2cDMx4iN3ATM)。

7. **安装使用**：商店应用在通过上架审核之后，任何Lark租户都可以在 [Lark应用中心](https://app.larksuite.com/) 发现和安装该应用。租户管理员在应用市场选择授权开通应用后，企业内成员的Lark工作台上将出现此应用，并能够开始使用。

:::note
   * 任何租户要安装商店应用，必须要由租户管理员在 [Lark管理后台](https://www.larksuite.com/admin) 进行审核和配置。
   * 租户管理员负责审核普通成员的应用安装、使用申请。
   * 一个商店应用可以被安装到多个租户内。
   * 商店应用需要持有企业对本应用的[访问凭证](/document/home/quickly-develop-three-party-approvals/creating-applications-and-requesting-permissions)，再以授权凭证访问企业在Lark上的数据。
:::


