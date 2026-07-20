---
document_id: '7260082411118739461'
directory_id: '7258197168736665606'
title: table.getRecordIdList
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getrecordidlist
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.getRecordIdList
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:56Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getrecordidlist
---

# table.getRecordIdList
获取记录id列表，该方法返回的记录id是无序的，若需要有序的请使用 视图 WidgetView 的 [view.getVisibleRecordIdList](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/view/view_getvisiblerecordidlist) 方法。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::




## 输出
Promise字符串数组。
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);

const res = await table.getRecordIdList();

```


### 返回示例
res:
```js
['recqrO5hl1', 'recpPHKX7F']
```
