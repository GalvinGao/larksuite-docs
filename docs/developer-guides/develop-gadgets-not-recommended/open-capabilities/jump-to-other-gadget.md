---
document_id: '6965379543683284998'
directory_id: '6907567266540371970'
title: 应用跳转
full_path: /uYjL24iN/uEjMxYjLxITM24SMyEjN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Open Capabilities
- Jump to Other Gadget
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:53Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEjMxYjLxITM24SMyEjN
---

# 应用跳转

小程序引擎支持通过调用[openSchema](/document/uYjL24iN/ukzN4IjL5cDOy4SO3gjM) 函数跳转到指定链接， 引擎会校验链接是否符合后台配置的白名单规则，不符合规则的链接将无法打开 (备注：PC端只有当external参数为false时才会校验)。

### 白名单限制
- 未添加白名单时, 将不允许 openSchema 调用打开
- 支持匹配链接的 scheme 和 host
- \*:\* 代表通配, 允许所有链接打开
- **配置后不需要重新发布版本，重新启动应用即可生效**

### 示例
- http://*  代表允许所有 scheme 为 http 的链接打开
- open.feishu.cn 代表允许 https://open.larksuite.com 下的链接打开
- http://open.feishu.cn 代表允许 http://open.feishu.cn 下链接打开
- https://applink.larksuite.com 代表允许Lark AppLink 的打开
- \*:\* 代表允许任意链接的打开

### 配置小程序跳转白名单
1. 登录开发者后台；
2. 从应用列表中选择相应的应用；
3. 确保应用已开启小程序能力；
4. 在左侧面板点击打开安全域名；
5. 在新页面 协议白名单 输入框中填写 URL 限制，并点击添加。


![小程序中文.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c700db68fd179add98e6d2694e4e21c4_AIzNRLGTFZ.png)
