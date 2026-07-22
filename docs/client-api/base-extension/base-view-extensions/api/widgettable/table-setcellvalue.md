---
document_id: '7260082411118706693'
directory_id: '7258197168736665606'
title: table.setCellValue
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_setcellvalue
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.setCellValue
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:45Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_setcellvalue
---

# table.setCellValue
设置单元格的值。

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
table.setCellValue(fieldId, recordId, cellValue)
```

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| fieldId | string | 是 | 字段id |
| recordId | string | 是 | 记录id |
| cellValue | [IOpenCellValue](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iopencellvalue) | 是 | 单元格的值 |



## 输出
Promise布尔值。为true的时候表示设置成功。
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const recordIds = await table.getRecordIdList();
const field = await table.getFieldByName('多行文本');

// 设置某个多行文本类型的字段
const res = await table.setCellValue(field.id,recordIds,[
    {
        type:'text',
        text:'123'
    }
])

```


### 返回示例
res:
```js
true
```
