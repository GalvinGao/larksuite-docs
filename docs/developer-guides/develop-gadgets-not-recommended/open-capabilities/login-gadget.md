---
document_id: '6966503476859912198'
directory_id: '6907567266540371970'
title: 登录小程序
full_path: /uYjL24iN/uETO5QjLxkTO04SM5kDN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Open Capabilities
- Login gadget
document_type: GuideDocumentType
updated_at: 2024-03-20T11:45:38Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETO5QjLxkTO04SM5kDN
---

# 登录小程序 

小程序可以通过Lark开放平台提供的登录能力，获取到Lark的用户身份标识。依据该标识，小程序应用能快速建立属于自己的用户体系。

:::note
如果需要获取用户的user_access_token调用OpenAPI，代表以该用户身份来执行/访问数据，那么额外还涉及到增量授权链路，参见 [小程序增量授权接入指南](/document/uYjL24iN/ukzMzUjL5MzM14SOzMTN/gadget-incremental-authorization-access-guide)
:::

## 登录流程时序图

![gadget_login_cn.jpg](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e03ff396e6ebacb6699cb450d50c473c_Kl2AEC37cW.jpg?height=1554&lazyload=true&maxWidth=750&width=2274)

## 操作步骤

1. 应用调用开放平台提供的登录接口 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 LarkV6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) ) 获取用户临时的登录凭证 code。
```
if (tt.requestAccess) {
  tt.requestAccess({
    scopeList: [],
    success: (res) => {
      // 用户授权成功后返回预授权码
      const { code } = res;
    },
    fail: (error) => {
      // 用户拒绝授权或者授权失败，返回相应的errno和errString
      const { errno, errString } = error;
    },
  });
} else { // 客户端版本过低，改为调用 tt.login
  tt.login({
    success: (res) => {
      // 登录成功后返回预授权码
      const { code } = res;
    },
    fail: (error) => {
      // 登录失败，返回相应的errno和errString
      const { errno, errString } = error;
    },
  });
}
```
2. 应用后端服务调用登录校验接口 [获取 user_access_token](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/oidc-access_token/create) 验证 code 的合法性，并通过[获取登录用户信息](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/user_info/get) 获取到用户身份。

3. 应用后端设置应用自身的登录态。
