---
document_id: '7072724425749151749'
directory_id: '7122028361539010566'
title: 三方审批接入准备
full_path: /ukTMukTMukTM/ukjNyYjL5YjM24SO2IjN/tripartite-approval-and-access-preparation
breadcrumb:
- Server API
- Approval
- Access guide
- Third-party approval integration
- Preparation for third-party approval integration
document_type: GuideDocumentType
updated_at: 2023-06-25T07:17:20Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukjNyYjL5YjM24SO2IjN/tripartite-approval-and-access-preparation
---

#  三方审批接入准备

## 时序图
三方系统对接Lark审批中心流程时序图，按照此流程可完成整个三方审批的对接。



![0819时序图.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0bfa29e72250c96d717e9502b2fbdb28_NAlFGij0Fx.png?lazyload=true&width=1484&height=1394)
## 准备
###  接入方式
审批中心通过[Lark开放平台](https://open.larksuite.com/) API 的方式开放这些能力，三方系统将审批数据通过开放平台接口推送到开放平台。
###  开放平台应用
使用审批中心接口需要申请开放平台企业自建应用，参考 [开发应用](/document/home/develop-a-bot-in-5-minutes/create-an-app)，审批中心API需要开启【访问审批】权限，新增权限后需要发布才生效。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/04bc1145c92a69192206cc0fc6f0823c_Of6kVNaw4k.png?lazyload=true&width=1280&height=681)

开放平台API 鉴权方式参考 [API访问凭证概述](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)，流程为：

1. 获取 app_id, app_secret
2. 使用 app_id, app_secret 获取 app_access_token
3. 使用 app_access_token 访问审批开放平台接口（自建应用的 app_access_token 等同于 tenant_access_token）


###  用户体系
审批中心使用开放平台用户体系，审批中心开放平台接口中的 user_id 为开放平台的user_id，含义参见[名词解释](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/terminology) ，获取 user_id 和 department_id 的方式可参考 [通讯录](/document/ukTMukTMukTM/uMTM4UjLzEDO14yMxgTN)
