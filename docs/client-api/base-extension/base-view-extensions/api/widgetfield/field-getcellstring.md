---
document_id: '7260082411118280709'
directory_id: '7258197168736731142'
title: field.getCellString
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/field/field_getcellstring
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetField
- field.getCellString
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:10Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/field/field_getcellstring
---

# field.getCellString
获取单元格的值 (字符串) 。

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
getCellString(recordId)
```

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| recordId | string | 是 | 记录id |



## 输出
Promise字符串。
## 示例代码

```js
    const selection = await bitable.base.getSelection();
    const table = await bitable.base.getTableById(selection.tableId);
    const filed = await table.getFieldByName('多行文本');
    const records = await table.getRecordIdList()

    const res = await filed.getCellString(records[0]) // 获取某个单元格的值（字符串）
    console.log(res)
```


