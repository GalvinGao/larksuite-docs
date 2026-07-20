---
document_id: '7073823165957619718'
directory_id: '7073460768595378181'
title: 打开SSO登录页
full_path: /uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-the-sso-login-page
breadcrumb:
- Developer Guides
- AppLink Protocol
- Supported protocol
- Open the SSO login page
document_type: GuideDocumentType
updated_at: 2022-03-11T16:42:53Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-the-sso-login-page
---

# 打开SSO登录页
::: note 
从Lark 3.35.0 版本开始支持。
:::

## 使用场景
在Lark客户端中打开租户在admin后台配置的SSO登录页

## 协议
[https://applink.larksuite.com/client/passport/sso_login?sso_domain=A&tenant_name=B](https://applink.larksuite.com/client/passport/sso_login?sso_domain=A&tenant_name=B)

## 参数

| 字段         | 必填           | 说明        | 
| --------- | --------------- | -------   | 
|**sso_domain** |    是      | 租户的域名，填写的是租户在admin后台配置的租户域名信息。当在admin后台改动租户的域名时，需要同步修改applink该参数值 | 
|**tenant_name** | 是 | 租户名，用于在切换租户时，客户端展示即将登录到的租户名称，一般填写公司名即可 | 



## 使用示例
用租户域名和租户名称打开SSO登录页。租户域名为"idptest.larksuite.com" ，租户名称为"测试租户"。
`https://applink.larksuite.com/client/passport/sso_login?sso_domain=idptest.larksuite.com&tenant_name=测试租户` 


因为Lark支持多user身份，当用户在Lark上登录了非当前applink对应的sso租户下的user时，会提示用户是否需要切换到sso租户下的user。具体的展示格式见下图

![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/13ba3fa628380dbca6b4886d909db5c0.png?lazyload=true&width=382&height=820)
