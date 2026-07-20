---
document_id: '7260082411118182405'
directory_id: '7258197168736698374'
title: base.getTableMetaList
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_gettablemetalist
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetBase
- base.getTableMetaList
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_gettablemetalist
---

# base.getTableMetaList
获取数据表元信息列表，不包含仪表盘，不保证顺序。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::


## 输出
Promise数据表元信息数组。
## 示例代码
### 调用示例

```js
const metaList = await bitable.base.getTableMetaList()
```


### 返回示例
metaList:
```js
[
  {
    id: 'tblkrAjKK1wEuhNf',
    name: '数据表'
  }
]
```
