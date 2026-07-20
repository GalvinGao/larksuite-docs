---
document_id: '7260082411118559237'
directory_id: '7258197168736698374'
title: base.isEditable
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_iseditable
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetBase
- base.isEditable
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:23Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_iseditable
---

# isEditable
是否是编辑模式。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::



## 输出
Promise布尔值，为true的时候表示处于编辑模式。
## 示例代码
### 调用示例

```ts
const isEditable = await bitable.base.isEditable()
```


### 返回示例
isEditable:
```ts
true
```
