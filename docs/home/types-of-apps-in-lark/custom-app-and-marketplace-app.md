---
document_id: '7026663896463802373'
directory_id: '7002892512470761478'
title: 自建应用与商店应用
full_path: /home/app-types-introduction/self-built-apps-and-store-apps
breadcrumb:
- Home
- Types of apps in Lark
- Custom app and marketplace app
document_type: GuideDocumentType
updated_at: 2023-07-28T03:43:40Z
source_url: https://open.larksuite.com/document/home/app-types-introduction/self-built-apps-and-store-apps
---

# 自建应用与商店应用

首先请思考：为什么要开发这款应用？

如果你开发的应用只会给同一租户内的用户使用，不需要分享到租户外部，请选择创建 **自建应用**。

一个自建应用的开发上线流程大致如下：

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/84c1a1f8a27354a28267486e26b20f7d_DeBc9eO30Q.png?lazyload=true&width=1640&height=817)
*(图1：自建应用的开发审核和安装流程示意)* 

<br>

从上图我们可以看到，自建应用是限制在单个租户内开发、审核和上架使用的。由于这个特性，应用的开发者、管理员和使用者都归属在一个租户内，对外部不可见。任何一个自建应用都不可被其他组织使用。Lark官方和其他第三方也不会介入到自建应用的管理流程中。点击这里开始 [创建自建应用](https://open.larksuite.com/app)

如果你希望开发的应用能被不同的租户安装使用，请选择开发**商店应用**。商店应用还支持收费结算机制。

一个商店应用的开发上架流程大致如下：

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4187797753b85a73616ab48b79873a85_xakmaiyoAp.png?lazyload=true&width=1640&height=674)

*(图2：商店应用的开发审核和安装流程示意)*

<br>

如上图所示，相比自建应用，商店应用的流程更加复杂一些：

-   任何一个组织如果希望开发商店应用并在[Lark应用目录](https://app.larksuite.com/)上架，首先必须通过 ISV 认证，可参考[应用服务商入驻](/document/uMzNwEjLzcDMx4yM3ATM/uUzNwEjL1cDMx4SN3ATM)
-   商店应用在开发时需要关注接口上的差异，比如访问凭证上有针对商店应用的特殊接口，参考[获取 app_access_token（应用商店应用）](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_access_token)和[获取 tenant_access_token（应用商店应用）](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token)等。
-   商店应用上架需要经过Lark官方运营的严格审核，参考[开发和上架应用商店应用](/document/uMzNwEjLzcDMx4yM3ATM/uYzNwEjL2cDMx4iN3ATM)。
-   商店应用在通过上架之后，任何Lark租户都可以在[Lark应用目录](https://app.larksuite.com/)发现和安装该应用。
-   任何租户要安装商店应用，必须要由租户管理员在[Lark管理后台](https://admin.feishu.cn)进行审核和配置。
-   商店应用在管理员完成审核之后才能被普通成员使用。
-   一个商店应用可以被安装到多个组织内。

##
