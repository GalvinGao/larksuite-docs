---
document_id: '7260081693314760710'
directory_id: '7258197168736649222'
title: view.getName
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/view/view_getname
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetView
- view.getName
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:10Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/view/view_getname
---

# view.getName
获取视图名字。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::



## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const viewMetaList = await table.getViewMetaList();
const view = await table.getViewById(viewMetaList[0].id);

const res = await view.getName(); // '表格'
```


### 返回示例
res:
```
'表格'
```
