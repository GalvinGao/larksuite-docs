---
document_id: '7260082411118886917'
directory_id: '7258197168736665606'
title: table.getCellString
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getcellstring
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.getCellString
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getcellstring
---

# table.getCellString
获取单元格的值 (字符串) 。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::


## 输入
```
getCellString(fieldId,recordId)
```

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| fieldId | string | 是 | 单元格的字段id |
| recordId | string | 是 | 单元格的记录id |



## 输出
Promise字符串。
## 示例代码

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const recordIds = await table.getRecordIdList(); // 随机获数据表所有行id
const fields = await table.getFieldList();// 获取数据表字段列表

const res = await table.getCellString(fields[0].id,recordIds[0]); 
console.log(res)

```


