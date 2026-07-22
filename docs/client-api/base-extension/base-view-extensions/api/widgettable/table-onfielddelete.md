---
document_id: '7260081693314482182'
directory_id: '7258197168736665606'
title: table.onFieldDelete
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_onfielddelete
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.onFieldDelete
document_type: GuideDocumentType
updated_at: 2024-05-15T02:45:08Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_onfielddelete
---

# table.onFieldDelete
监听字段添加事件，将返回一个取消监听函数。

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
onFieldDelete((event) => {})
```

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| event | <code>{<br>  "name": string,<br>  "type": "publish",<br>  "data": {<br>    "tableId": string,<br>    "fieldId": string,<br>    "fieldType": number<br>  },<br>}</code> | 否 | 被删除字段的相关信息 |



## 输出
取消监听的函数。
## 示例代码

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);

const off = table.onFieldDelete((event) => {
  off();
  console.log('event:', event);
})

const fieldId = await table.addField({ // 新增字段。
  type: FieldType.Text,
  name: '新增-2'
})

table.deleteField(fieldId)//删除字段。
```


