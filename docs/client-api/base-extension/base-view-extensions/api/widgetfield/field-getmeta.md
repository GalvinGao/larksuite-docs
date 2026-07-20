---
document_id: '7260082411118837765'
directory_id: '7258197168736731142'
title: field.getMeta
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/field/field_getmeta
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetField
- field.getMeta
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:18Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/field/field_getmeta
---

# field.getMeta
获取字段元信息。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::



## 输出
Promise[IFieldMeta](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/ifieldmeta)对象。
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const field = await table.getFieldByName('多行文本');

const res = await field.getMeta();
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
