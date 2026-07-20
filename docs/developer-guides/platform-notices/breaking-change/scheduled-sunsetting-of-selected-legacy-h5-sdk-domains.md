---
document_id: '7315336472487788549'
directory_id: '7077912803110010885'
title: H5 SDK 的部分历史域名即将下线
full_path: /uAjLw4CM/ugTN1YjL4UTN24CO1UjN/breaking-change/scheduled-sunsetting-of-selected-legacy-h5-sdk-domains
breadcrumb:
- Developer Guides
- Platform Notices
- Breaking change
- Scheduled sunsetting of selected legacy H5 SDK domains
document_type: GuideDocumentType
updated_at: 2023-12-25T06:16:55Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/breaking-change/scheduled-sunsetting-of-selected-legacy-h5-sdk-domains
---

# H5 SDK 的部分历史域名即将下线

## 变更事项

为了提升服务安全性，我们将下线 H5 SDK 使用的部分历史 CDN 域名。下线后，引用这些历史域名的网页应用 H5 SDK 将会受到影响 ，应用内无法调用 H5 SDK 内的方法。
  
**受影响的历史域名列表**：<br>
s0.bytecdn.cn<br>
s0.pstatp.com<br>
s0z.pstatp.com<br>
s1.pstatp.com<br>
s2.pstatp.com<br>
s3.bytecdn.cn<br>
s3.pstatp.com<br>
s3a.bytecdn.cn<br>
s3b.bytecdn.cn<br>
s3a.pstatp.com<br>
s3b.pstatp.com<br>
s4.pstatp.com<br>
s6.pstatp.com<br>
s9.pstatp.com<br>
<br>
是否跟版：不跟版<br>
预计生效版本：-<br>
历史域名预计下线时间：**2024-01-25**

## 解决方案

为不影响业务的正常运行，请注意检查你的网页应用是否引用了上述域名，并尽快将相关域名替换为最新版本的 H5 SDK 域名。
- **网页应用 H5 SDK**：[参考网页应用开发指南](/document/uYjL24iN/uMTMuMTMuMTM/introduction)

```JS
<script
type= "text/javascript"
src= "https://lf16-oversea.goofy-cdn.com/goofy-va/lark/op/h5-js-sdk-1.5.26.js"
></script> 
```

  
若你未能及时确认并调整，域名下线后，相关场景的业务将无法正常运行。 <br>
开放平台预计将于 **2024年1月25日** 完成相关历史域名的下线。请于 **2024年1月25日前** 确认以上信息，并根据情况做好相应适配。<br>
如需适配协助，请联系技术支持。
