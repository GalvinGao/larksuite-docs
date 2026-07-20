---
document_id: '7260082411118936069'
directory_id: '7258197168736665606'
title: table.getViewMetaList
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getviewmetalist
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.getViewMetaList
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getviewmetalist
---

# table.getViewMetaList
获取视图元信息列表。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::




## 输出
字段元信息对象数组。
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);

const res = await table.getViewMetaList();
console.log(res)
```


### 返回示例
res:
```js
[
  {
    "name": "表格",
    "id": "vewDUmweGB",
    "type": 1
  }
]
```
