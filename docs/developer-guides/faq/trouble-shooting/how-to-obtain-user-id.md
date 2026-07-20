---
document_id: '7166859792995958790'
directory_id: '7161758872830820357'
title: 如何获取自己的 User ID？
full_path: /uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id
breadcrumb:
- Developer Guides
- FAQ
- Trouble Shooting
- How to Obtain User ID
document_type: GuideDocumentType
updated_at: 2024-03-18T02:16:21Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id
---

# 如何获取自己的 User ID？



## 前提条件

- [已开通「通过手机号或邮箱获取用户 ID」权限（`contact:user.id:readonly`）](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-fix-the-99991672-error)
- [已开通「获取用户 User ID」权限（`contact:user.employee_id:readonly`）](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-fix-the-99991672-error)



## 1. 打开 API 调试台，并找到「通过手机号或邮箱获取用户 ID」API

打开Lark开放平台 [API 调试台](https://open.larksuite.com/api-explorer?from=op_doc_tab)，并在左侧 API 目录中找到「通讯录」下的「通过手机号或邮箱获取用户 ID」，点击该 API 切换当前调试 API 为「通过手机号或邮箱获取用户 ID」。

> 可以在API 列表顶部的搜索框输入「通过手机号或邮箱获取用户 ID」来快速定位。

![cn01_副本.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ae820537850ea599f4ffa37122234e5b_ybDRcXjHdm.png?lazyload=true&width=3050&height=1411)

### 2. 获取鉴权凭证，并设置参数

1. 点击 API 调试台左侧「查看鉴权凭证」中 tenant_access_token 中的「点击获取」（如果之前已经获取过，则可以点击刷新按钮刷新鉴权凭证。

2. 点击右侧参数列表，将查询参数 Tab 中的 *user_id_type* 参数设置为 *user_id*。

3. 切换至请求体 Tab，将请求体中的 ID 删除，并修改 *mobiles* 参数，设为你自己的手机号。

![cn02_副本.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a25396b274094c614ece2a19566093db_8m74CsepTO.png?lazyload=true&width=3056&height=1455)

### 3. 调试，并获得 User ID

点击右侧「开始调试」，调用接口。调用成功后，在下方响应体中即可拿到你自己的 User ID。

![cn03_副本.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/24ef07585ca8cc613a8e626d2f516740_3UXCW4FcYy.png?lazyload=true&width=3030&height=1480)
