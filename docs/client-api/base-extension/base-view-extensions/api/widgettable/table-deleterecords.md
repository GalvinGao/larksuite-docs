---
document_id: '7281229133446037509'
directory_id: '7258197168736665606'
title: table.deleteRecords
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_deleterecords
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.deleteRecords
document_type: GuideDocumentType
updated_at: 2023-09-22T02:26:44Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_deleterecords
---

# table.deleteRecords
批量删除记录，最多5000条。

## 权限要求
:::html
<md-alert type="warn">
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>

:::


## 输入
```
deleteRecords(recordIdList)
```

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| recordIdList | string[] | 是 | 需要删除的记录id列表 |




## 输出
Promise布尔值。
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const recordIds = await table.getRecordIdList(); // 获取所有记录id

const res = await table.deleteRecords(recordIds.slice(0,500)); // 删除前500条记录
console.log(res) // true
```
  
