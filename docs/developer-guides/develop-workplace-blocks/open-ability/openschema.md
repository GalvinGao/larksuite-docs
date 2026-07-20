---
document_id: '7180270043522596870'
directory_id: '7180165099250925573'
title: 应用跳转
full_path: /uAjLw4CM/uYjL24iN/block/guide/open-ability/openschema
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Open Ability
- openSchema
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:00Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/guide/open-ability/openschema
---

# 应用跳转

小组件引擎支持通过调用 [openSchema](/document/uAjLw4CM/uYjL24iN/block/api/navigator/openschema) 函数跳转到指定链接， 引擎会校验链接是否符合后台配置的白名单规则，不符合规则的链接将无法打开。

:::html
<md-alert>PC 端只有当 external 参数为 false 时才会校验。</md-alert>
:::


## 配置小组件跳转白名单
1. 登录开发者后台；
2. 从应用列表中选择相应的应用；
3. 确保应用已开启小组件能力；
4. 在左侧面板点击打开安全域名；
5. 在新页面 协议白名单输入框中填写 URL 限制，并点击添加。

![小组件中文.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4da63d17a75556fbdc74db1a2ccae142_MJE5HEvEjP.png?lazyload=true&width=3280&height=1850)


## 白名单规则
- 未添加白名单时, 将不允许 openSchema 调用打开。
- 支持匹配链接的 scheme 和 host。
- **配置后不需要重新发布版本，重新启动应用即可生效**。

## 示例
- `http://*` 代表允许所有 scheme 为 http 的链接打开。
- `open.feishu.cn` 代表允许 https://open.larksuite.com 下的链接打开。
- `http://open.feishu.cn` 代表允许 http://open.feishu.cn 下的链接打开。
- `https://applink.larksuite.com` 代表允许Lark AppLink 打开。
- `*:*` 代表允许任意链接打开。
