---
document_id: '7166859792995926022'
directory_id: '7161758872830820357'
title: 如何获取自己的 Open ID？
full_path: /uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid
breadcrumb:
- Developer Guides
- FAQ
- Trouble Shooting
- How to Obtain OpenID
document_type: GuideDocumentType
updated_at: 2024-03-18T02:16:13Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid
---

# 如何获取自己的 Open ID？

### 前提条件

- [已开通「通过手机号或邮箱获取用户 ID」权限（`contact:user.id:readonly`）](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-fix-the-99991672-error)

### 1. 打开 API 调试台，并找到「通过手机号或邮箱获取用户 ID」API

打开 Lark 开放平台 [API 调试台](https://open.larksuite.com/api-explorer?from=op_doc_tab)，并在左侧 API 目录中找到「通讯录」下的「通过手机号或邮箱获取用户 ID」，点击该 API 切换当前调试 API 为「通过手机号或邮箱获取用户 ID」。

> 可以在API 列表顶部的搜索框输入「通过手机号或邮箱获取用户 ID」来快速定位。

![01_副本.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/522fa767cf01b32b7a1567233cd8e18c_WAdULaKVj2.png?lazyload=true&width=3050&height=1413)

### 2. 获取鉴权凭证，并设置参数

1. 点击 API 调试台左侧「查看鉴权凭证」中 tenant_access_token 中的「点击获取」（如果之前已经获取过，则可以点击刷新按钮刷新鉴权凭证。

2. 点击右侧参数列表，将查询参数 Tab 中的 *user_id_type* 参数设置为 *open_id*。

3. 切换至请求体 Tab，将请求体中的 ID 删除，并修改 *mobiles* 参数，设为你自己的手机号。


![02_副本.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5574a41f78aa1e551940352c91d49db8_tvffPsQ365.png?lazyload=true&width=3056&height=1458)


### 3. 调试，并获得 OpenID

点击右侧「开始调试」，调用接口。调用成功后，在下方响应体中即可拿到你自己的 Open ID。


![03_副本.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/37ee7ad5211de3b4c880e8852c157138_UjW3FfvLnt.png?lazyload=true&width=3030&height=1418)
