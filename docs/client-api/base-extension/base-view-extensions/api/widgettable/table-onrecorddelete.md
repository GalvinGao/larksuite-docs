---
document_id: '7260081693314465798'
directory_id: '7258197168736665606'
title: table.onRecordDelete
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_onrecorddelete
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.onRecordDelete
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:37Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_onrecorddelete
---

# table.onRecordDelete
监听记录删除事件，将返回一个取消监听函数。

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
onRecordDelete((event) => {})
```

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| event | <code>{<br>  "name": string,<br>  "type": "publish",<br>  "data": string[],<br>}</code> | 否 | data为被删除记录的recordId数组 |



## 输出
取消监听的函数。
## 示例代码

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const recordIds = await table.getRecordIdList();

const off = table.onRecordDelete((event) => { // 监听记录删除事件
  off();
  console.log('event:', event);
})

table.deleteRecord(recordIds[0]) // 删除记录
```
