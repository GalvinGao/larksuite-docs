---
document_id: '7260081693314711558'
directory_id: '7258197168736698374'
title: base.getTableList
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_gettablelist
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetBase
- base.getTableList
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_gettablelist
---

# base.getTableList
获取数据表实例列表，不包含仪表盘。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::


## 输出
Promise[数据表实例](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iwidgettable)数组。


## 示例代码

```js
await bitable.base.getTableList()
```
