---
document_id: '7260081693314678790'
directory_id: '7258197168736665606'
title: table.getName
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getname
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.getName
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:08Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getname
---

# table.getName
获取表名。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::




## 输出
promise字符串。
## 示例代码

```js
const table = await bitable.base.getTableByName('数据表')

const name = await table.getName()

console.log(name) // 数据表
```

