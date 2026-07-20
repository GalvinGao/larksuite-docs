---
document_id: '7295682588634873861'
directory_id: '7294197569669185541'
title: 步骤二：获取访问凭证
full_path: /home/quickly-build-a-feishu-calendar-schedule/step-2-obtain-access-credentials
breadcrumb:
- Home
- Quickly Build Calendar Events
- 'Step 2: Obtain access credentials'
document_type: GuideDocumentType
updated_at: 2023-10-31T08:32:43Z
source_url: https://open.larksuite.com/document/home/quickly-build-a-feishu-calendar-schedule/step-2-obtain-access-credentials
---

# 步骤二：获取访问凭证

本教程以 API 调试台工具为例，介绍如何使用Lark开放平台日历服务的 API ，将企业原有的工作日程同步至Lark日历中。你可以参考 API 调用逻辑完善自己的服务端代码。在调用 API 之前，你需要先通过调试台获取访问凭证。

:::note
由于本教程使用的应用为测试版本，因此后续所有的 API 调用、Lark客户端操作，均使用测试企业中的人员账号登录进行。
:::

## 操作步骤

1. 打开 [API调试台](https://open.larksuite.com/api-explorer)。

2. 在页面左上角点击 **切换应用**，并选择步骤一中已创建的应用。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8a788b1eec4d1392918c75d4f1857156_56RnGgxlEn.png?height=1112&lazyload=true&maxWidth=600&width=2296)

3. 在左侧 **查看鉴权凭证** 区域，获取应用的 **tenant_access_token**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c473247e5679bd2179f82fd68e63b7ca_h8A5bdGKsO.png?height=1048&lazyload=true&maxWidth=600&width=2328)
