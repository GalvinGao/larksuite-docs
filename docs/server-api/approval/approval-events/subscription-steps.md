---
document_id: '7199928167141392389'
directory_id: '7021712771402661894'
title: 订阅步骤
full_path: /ukTMukTMukTM/uIDO24iM4YjLygjN/event/subscription-steps
breadcrumb:
- Server API
- Approval
- Approval Events
- Subscription steps
document_type: GuideDocumentType
updated_at: 2023-02-15T03:03:47Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uIDO24iM4YjLygjN/event/subscription-steps
---

# 订阅步骤

### 1. 在应用后台中配置事件订阅回调URL

- 在「开发者后台」-「事件订阅」注册事件回调地址。具体步骤请参照开放平台文档：[事件订阅概述](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)。
### 2. 开通应用权限
- 在「开发者后台」-「权限管理」中搜索"approval",将搜索出来的权限打开。

### 3. 订阅审批事件
- 在「开发者后台」-「事件订阅」中，添加事件，例如可以订阅“审批实例状态变更”或"审批任务状态变更"事件。

### 4. 订阅需要监听的审批定义（Approval Code）

在应用后台订阅审批事件后，仍需要再次通过审批开放接口订阅指定的审批定义，才会收到审批事件。

* [订阅审批事件](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/subscribe)
* [取消订阅审批事件](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/unsubscribe)

审批接口权限认证依赖 tenant_access_token，获取方式如下：

* [获取 app_access_token（企业自建应用）](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_access_token_internal)
* [获取 tenant_access_token（应用商店应用）](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token)
