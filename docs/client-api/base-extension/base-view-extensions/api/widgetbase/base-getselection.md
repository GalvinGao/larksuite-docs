---
document_id: '7260082411118329861'
directory_id: '7258197168736698374'
title: base.getSelection
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_getselection
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetBase
- base.getSelection
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:16Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_getselection
---

# base.getSelection
获取当前所选数据表的相关信息。在record view和多维表格扩展脚本上返回值略有不同。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::
## 示例代码

### 调用示例

```js
await bitable.base.getSelection()
```


### 返回示例
在扩展脚本上的某些字段为null，使用时需注意判断值的类型:
```js
{
  baseId: "QtTUb1WewaBmtcsyafTcoEpUnzf",
  fieldId: null,
  recordId: null,
  tableId: "tblrWshG3HMh3BlM",
  viewId: null,
}
```
