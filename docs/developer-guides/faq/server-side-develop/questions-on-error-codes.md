---
document_id: '7069302182252003333'
directory_id: '7069296428547686405'
title: 错误码相关
full_path: /ugTN1YjL4UTN24CO1UjN/uEDO1YjLxgTN24SM4UjN
breadcrumb:
- Developer Guides
- FAQ
- Server-side develop
- Questions on error codes
document_type: GuideDocumentType
updated_at: 2022-03-04T11:13:26Z
source_url: https://open.larksuite.com/document/ugTN1YjL4UTN24CO1UjN/uEDO1YjLxgTN24SM4UjN
---

# 错误码相关
:::note
你可以在此搜索你碰到的服务端API错误码及排查建议 [服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)

:::

**1. 我碰到了一个在开放平台文档里没有的错误码，怎么办？**

答：很抱歉造成了不便，遇到此类情况可以[联系客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)查询。同时我们也会尽快更新相关代码说明。

你还可以通过点击Lark桌面端左上角个人头像，选择**帮助与客服**进行联系。
<br>
<br>

**2. 使用身份验证[获取用户身份](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/authen/refresh_access_token) 接口返回20007报错码是什么原因?**

答：
- [获取用户身份信息接口](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/authen/refresh_access_token)返回code为1007或者20007，说明code非法，请确保 code 没有重复消费或过期消费（code只能使用一次且5分钟内有效）
- 小程序 tt.login() 获取的 code 不可用于服务端 API 获取用户信息

