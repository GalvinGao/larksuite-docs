---
document_id: '7180270043523104774'
directory_id: '7180165099250876421'
title: 本地缓存
full_path: /uAjLw4CM/uYjL24iN/block/guide/basic-ability/storage
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Basic Ability
- LocalStorage
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/guide/basic-ability/storage
---

# 本地缓存

每个小组件 (Block) 都可以有自己的本地缓存，可以通过以下 API 对本地缓存进行读写和清理。

-   [setStorage](/document/uAjLw4CM/uYjL24iN/block/api/data-cache/setstorage) 
-   [getStorage](/document/uAjLw4CM/uYjL24iN/block/api/data-cache/getstorage) 
-   [removeStorage](/document/uAjLw4CM/uYjL24iN/block/api/data-cache/removestorage) 
-   [clearStorage](/document/uAjLw4CM/uYjL24iN/block/api/data-cache/clearstorage) 
-   [getStorageInfo](/document/uAjLw4CM/uYjL24iN/block/api/data-cache/getstorageinfo) 

## 隔离策略

-   每个小组件的 storage 会以「用户ID维度 + blockTypeID维度」进行自动隔离，无法相互访问。
-   每个「用户ID维度 + blockTypeID维度」的 storage 上限为10MB。
-   更细维度的隔离策略（比如 blockID 维度），需要业务自行管理。

## 清理策略

目前本地缓存的清理都必须**依赖业务**通过[removeStorage](/document/uAjLw4CM/uYjL24iN/block/api/data-cache/removestorage)或[clearStorage](/document/uAjLw4CM/uYjL24iN/block/api/data-cache/clearstorage)处理。
