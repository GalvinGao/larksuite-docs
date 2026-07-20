---
document_id: '7260081693314777094'
directory_id: '7258197168736731142'
title: field.getProxyType
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/field/field_getproxytype
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetField
- field.getProxyType
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:22Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/field/field_getproxytype
---

# field.getProxyType
获取 公式/查找引用字段 所代理的字段的字段类型[FieldType](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/fieldtype)。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::



## 输出
Promise 字段类型[FieldType](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/fieldtype)或null。
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const filed = await table.getFieldByName('查找引用'); // 获取查找引用字段。该字段引用了某个多行文本字段;

const res = await filed.getProxyType()
```


### 返回示例
res:
```js
1
```
