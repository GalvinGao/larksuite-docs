---
document_id: '7166859792995942406'
directory_id: '7161758872830820357'
title: 如何解决 tenant token invalid (99991663) 错误
full_path: /uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-fix-99991663-error
breadcrumb:
- Developer Guides
- FAQ
- Trouble Shooting
- How to resolve tenant token invalid (999991663) error
document_type: GuideDocumentType
updated_at: 2024-03-18T02:16:25Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-fix-99991663-error
---

# 如何解决 tenant token invalid (99991663) 错误

##  问题现象

在调用 OpenAPI 时，有可能会出现 99991663 错误，该报错表现为接口返回体中 Code 为 99991663。

接口报错参考如下：

```
{
  "code": 99991663,
  "msg": "tenant token invalid",
  "error": {
    "subcode": 20109004,
    "log_id": "XXXXXX"
  }
}
```


## 可能原因

该报错是因为开发者本次调用过程中使用的 Tenant Access Token 已经失效或有误，Lark开放平台无法判断当前请求是否来自一个可信的用户，因此拦截了情况。

## 解决方法

### API 调试台场景

如果你使用Lark开放平台提供的 API 调试台调用接口，可以通过 API 调试台自带功能刷新 Tenant Access Token，并使用新的 Tenant Access Token 调用 OpenAPI。

点击[API 调试台](https://open.larksuite.com/api-explorer?from=op_doc_tab)左侧的「查看鉴权凭证」，并点击 tenant_access_token 选项中的刷新按钮，刷新 Tenant Access Token。刷新完成后，即可直接调用接口，API 调试台将会请求中的 Token 替换为最新获取的 Token。

![CN01_副本.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5c7413aae7711c8103a042978f41f39a_qpQOr1yOwB.png?lazyload=true&width=1852&height=641)

### API 调用场景

:::html
<md-alert type="tip">
Lark开放平台提供了服务端 SDK ，提供了完整的访问凭证（AccessToken）生命周期管理能力，无需开发者自己获取并刷新访问凭证。点击[了解如何使用Lark开放平台服务端 SDK]((/document/ukTMukTMukTM/uETO1YjLxkTN24SM5UjN))。
</md-alert>
:::

如果你是自行编写代码获取 Token，则可以自行参考 [获取访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM#a8683ac2) 提供的方法，再次调用Lark开放平台获取 Tenant Acess Token 的接口，并使用最新的 Tenant Access Token 请求 OpenAPI。

