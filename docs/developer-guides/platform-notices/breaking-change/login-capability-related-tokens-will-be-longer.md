---
document_id: '7078255338885677062'
directory_id: '7077912803110010885'
title: 登录能力相关凭证的数据长度变更
full_path: /uAjLw4CM/ugTN1YjL4UTN24CO1UjN/breaking-change/login-capability-related-tokens-will-be-longer
breadcrumb:
- Developer Guides
- Platform Notices
- Breaking change
- Login capability-related tokens will be longer
document_type: GuideDocumentType
updated_at: 2022-05-05T09:24:34Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/breaking-change/login-capability-related-tokens-will-be-longer
---

# 登录能力相关凭证的数据长度变更
### 变更事项
为了防止网络攻击，提高服务的安全性，我们将增加开放平台应用登录能力相关凭证的数据长度，相关凭证包括登录预授权码（code）、用户身份访问凭证（user_access_token）和刷新凭证（refresh_token）。优化后，上述凭证的数据位数最多可达到 64 字节。


是否跟版：不跟版<br>
预计生效时间：2022-5-20<br>

### 潜在影响
若你的应用使用了任何形式的登录开放能力（如网页登录、小程序登录），请尽快确认存储登录过程中获取的上述三项凭证使用的存储空间（如数据库字段长度、缓存长度、内存字符串长度等）是否均可容纳 64 字节字符串，适配详细步骤请见[详细说明](https://bytedance.feishu.cn/docx/doxcnrkd9sExn0MZIkMt90FYB2x)
。<br>


### 解决方案
- 如果上述凭证的存储空间均可容纳 64 字节字符串，则无需进行任何调整；
- 如果上述任一凭证的存储空间无法容纳 64 字节字符串，请进行相应调整，使其存储空间可容纳至少 64 字节。<br><br>
若你未能及时确认并调整存储空间，凭证长度变更后，存储空间不足将会导致数据丢失。<br><br>
我们预计在 2022年5月20日 完成升级，请于 2022年5月16日前 确认以上信息，并根据情况做好相应适配。<br>
<br> 如需适配协助，请洽客服支持
