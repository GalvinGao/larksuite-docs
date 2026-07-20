---
document_id: '7348410416865755142'
directory_id: '7346117913857048581'
title: 登录小程序( 旧版本 )
full_path: /uYjL24iN/ukzMzUjL5MzM14SOzMTN/old-gadget-login
breadcrumb:
- Developer Guides
- Deprecated Guides
- Gadgets
- Gadget login (old version)
document_type: GuideDocumentType
updated_at: 2024-03-20T11:45:49Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukzMzUjL5MzM14SOzMTN/old-gadget-login
---

# 登录小程序 

小程序可以通过Lark开放平台提供的登录能力，获取到Lark的用户身份标识。依据该标识，小程序应用能快速建立属于自己的用户体系。

## 登录流程时序图

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d27134675b9e055845f10c844954b409_oSWJBEIwyx.png?height=795&lazyload=true&maxWidth=750&width=1153)

## 操作步骤

1. 应用调用开放平台提供的登录接口 [tt.login](/document/uYjL24iN/uYzMuYzMuYzM) 获取用户临时的登录凭证 code。

2. 应用后端服务调用登录校验接口 [code2session](/document/uYjL24iN/ukjM04SOyQjL5IDN) 验证 code 的合法性，并获取到用户身份。

3. 应用后端设置应用自身的登录态。
