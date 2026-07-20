---
document_id: '7132687771717910533'
directory_id: '7072190414392344582'
title: 概述
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/getting-start
breadcrumb:
- Server API
- Docs
- Wiki
- Overview
document_type: GuideDocumentType
updated_at: 2025-08-14T06:32:49Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/getting-start
---

# 概述
你可以通过知识库 API，来自动化管理你的知识库。
:::note
在调用知识库 API 之前，请确保你的应用已经按需申请了以下权限并发布：
- `wiki:wiki`: 可以对知识库进行增删改查
- `wiki:wiki.readonly`: 可以读取知识库内容，无法编辑或修改知识库


相关说明详见：[应用权限](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN)
:::


# 名词解释

## 知识空间
知识空间是知识库的基本组成单位，是企业根据需要搭建的不同类别的知识体系，由多个具有层级和所属关系的文档页面构成。每个知识空间，都有唯一的一个 space_id 作为标识。

**可以通过以下任一方法获取知识库的 space_id：**
- 调用 [获取知识空间列表](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/list)，从返回值中获取；
- 如果你是知识库管理员，可以进入知识库设置页面，复制地址栏的数字部分;

### 相关方法
-   [创建知识空间](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/create)
-   [获取知识空间列表](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/list)
-   [获取节点信息](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/get_node)
-   [更新知识空间设置](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-setting/update)


## 节点
每个知识空间都是由一系列文档页面构成，这些文档页面的从属关系会以树状展示，被称为页面树。每个节点都有唯一的一个 wikiToken 作为标识；

### 相关方法
-   [创建节点](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/create)
-   [获取节点信息](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/get_node)
-   [获取子节点列表](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/list)
-   [添加云文档至知识空间](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/move_docs_to_wiki)


## 成员
知识空间的成员有管理员、普通成员两种类型，支持通过[添加知识空间成员](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-member/create)，将一个应用设置为知识库成员。



