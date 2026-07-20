---
document_id: '7074952334765703174'
directory_id: '7073442394955546629'
title: 获取访问凭证
full_path: /home/synchronize-corporate-organizational-structure-to-feishu/obtain-the-application-access-token
breadcrumb:
- Home
- Synchronize corporate organizational structure to Lark
- Obtain access token
document_type: GuideDocumentType
updated_at: 2023-05-16T08:43:58Z
source_url: https://open.larksuite.com/document/home/synchronize-corporate-organizational-structure-to-feishu/obtain-the-application-access-token
---

# 获取访问凭证

在调用Lark开放平台接口之前，你需要先获取对应应用的访问凭证（access_token）。访问凭证代表应用得到了平台、租户和用户的授权。在本文中，你将使用API调试台获取自建应用的访问凭证**tenant_access_token**。

## 操作步骤

1. 登录[开发者后台](https://open.larksuite.com/app)，然后单击应用名称进入应用详情页。

2. 在应用详情页，选择**凭证与基础信息**，然后复制应用的**App ID**和**App Secret**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/564c382a6e49fcc442b8e38d77768e09_0as6Ux22ki.png?lazyload=true&width=2392&height=1042)

3. 打开[API调试台](https://open.larksuite.com/api-explorer)工具，然后单击切换应用，选择进行测试的应用。

4. 在左侧**查看鉴权凭证**栏，获取应用的**tenant_access_token**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/347e2d2ffc22dd3fb25d63789b178508_tlNn0dUzNN.png?lazyload=true&width=2040&height=1004)

5. （可选）如果你需要获取其它应用的访问凭证**tenant_access_token**，可以参考以下操作。
    
    1. 在API列表依次选择**鉴权管理** > **访问凭证** > ******自建应用** **获取tenant_access_token**。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b3efd64cb3485a3dbccbb1c69a025c79_uAzxEvKFhy.png?lazyload=true&width=1680&height=1418)
    
    2. 单击请求体，然后在请求体中填写应用的**AppID**和**AppSecret**。
    
    3. 单击右上角**开始调试**。
        
        调用成功的结果类似如下：
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4223deca0c6417ad25684c01381ba172_cWrN2otRZa.png?lazyload=true&width=2754&height=1496)
