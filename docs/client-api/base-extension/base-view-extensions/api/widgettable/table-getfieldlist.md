---
document_id: '7260082411118444549'
directory_id: '7258197168736665606'
title: table.getFieldList
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getfieldlist
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.getFieldList
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:08Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getfieldlist
---

# base.getFieldList
获取字段实例列表。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::


## 输出
Promise[字段实例](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iwidgetfield)数组。


## 示例代码

```js
const table = await bitable.base.getTableByName('数据表')
const fieldList = await table.getFieldList()
console.log(fieldList)
```
