---
document_id: '7270779605450457094'
directory_id: '7270719284443545605'
title: 安全配置
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/08-cloud-doc-block-security-configuration/cloud-doc-block-security-configuration
breadcrumb:
- Developer Guides
- Develop Docs Add-ons
- Cloud doc block security configuration
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:24Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/08-cloud-doc-block-security-configuration/cloud-doc-block-security-configuration
---

# 安全配置

## 背景说明

出于**提供更好的数据安全保障**，新版云文档小组件提供安全配置能力，包括 [CSP](https://developer.mozilla.org/zh-CN/docs/Web/HTTP/CSP)和 **API权限管理** 部分。应用需要申请资源访问权限，并经过开放平台或租户管理员审核后，才能使用权限绑定的开放能力。简单来说，API 权限决定了应用能使用哪些Lark的开放能力。
## 配置方法

### [CSP](https://developer.mozilla.org/zh-CN/docs/Web/HTTP/CSP) 请求访问限制配置

1. CSP 能力开启后会对非域名白名单的请求做拦截，会导致小组件不可使用，打开云文档控制台会报如下错误。
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6b0036ff9da80eaf3bdc823ded08f5d4_ijfMQtzaUJ.jpeg?lazyload=true&width=3082&height=158)
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5a8447f9943098a6ac7c320f0da5a923_yhLXbkmHoX.jpeg?lazyload=true&width=3052&height=164)
2. 需要去开发者后台-安全设置-服务器域名白名单，添加服务器域名白名单，然后发布新版本，审核通过即可。
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/bd08c2732fa96392e7e1cee714227626_x6ucaLirh4.jpeg?lazyload=true&width=3762&height=2124)
  
### 云文档小组件 API 权限约束配置引导

需要在开发者后台，权限管理，选择云文档目录，选择 **【创建及编辑新版文档】和 【查看新版文档】** 两个权限点位，开通权限，并发布提交审核。未开通上述权限点位的应用，API 将不可调用。API 与权限点位的对应关系说明，参考：[API 概览](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/05-api-doc)
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/30a00dce25cf0ca8b3cafbc0d849ae60_cn7Hlpmhao.jpeg?lazyload=true&width=4572&height=2146)
