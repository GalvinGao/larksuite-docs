---
document_id: '7260082411118510085'
directory_id: '7258197168736731142'
title: field.getType
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/field/field_gettype
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetField
- field.getType
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:25Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/field/field_gettype
---

# field.getType
获取字段的字段类型[FieldType](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/fieldtype)。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::



## 输出
Promise 字段类型[FieldType](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/fieldtype)。
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const filed = await table.getFieldByName('查找引用'); // 获取查找引用字段。该字段引用了某个多行文本字段;

const res = await filed.getType()
```


### 返回示例
res:
```js
19
```
