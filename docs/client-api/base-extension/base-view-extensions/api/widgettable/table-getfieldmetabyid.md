---
document_id: '7260081693314662406'
directory_id: '7258197168736665606'
title: table.getFieldMetaById
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getfieldmetabyid
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.getFieldMetaById
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:00Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getfieldmetabyid
---

# base.getFieldMetaById
获取某个 field 元信息。

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
getFieldMetaById(fieldId)
```

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| fieldId | string | 是 | 字段的id |



## 输出
Promise字段元信息对象。
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId); // 获取当前table
const fieldList = await table.getFieldList();// 获取当前table下所有字段

const res = await table.getFieldMetaById(fieldList[0].id);
```


### 返回示例
res:
```js
{
  "id": "fldgyVuQbo",
  "type": 1,
  "name": "多行文本",
  "property": {}
}
```
