---
document_id: '7260081693314580486'
directory_id: '7258197168736731142'
title: field.getName
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/field/field_getname
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetField
- field.getName
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:25Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/field/field_getname
---

# field.getName
获取字段名。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::



## 输出
Promise字符串：该字段的字段名。
## 示例代码
### 调用示例

```js
    const selection = await bitable.base.getSelection();
    const table = await bitable.base.getTableById(selection.tableId);
    const filed = await table.getFieldByName('查找引用');

    const res = await filed.getName()
```


### 返回示例
res:
```js
'查找引用'
```
