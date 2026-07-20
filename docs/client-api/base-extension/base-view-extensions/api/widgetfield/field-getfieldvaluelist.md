---
document_id: '7260081693314564102'
directory_id: '7258197168736731142'
title: field.getFieldValueList
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/field/field_getfieldvaluelist
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetField
- field.getFieldValueList
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:14Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/field/field_getfieldvaluelist
---

# field.getFieldValueList
获取整列不为空的 cellValue。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::



## 输出
Promise数组，数组元素为[IOpenCellValue](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iopencellvalue)。
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const filed = await table.getFieldByName('多行文本');

const res = await filed.getFieldValueList()

```


### 返回示例
res:
```js
[
  {
    "record_id": "rec1i6gIHj",
    "value": [
      {
        "type": "text",
        "text": "1"
      }
    ]
  },
  {
    "record_id": "rec8Do63xI",
    "value": [
      {
        "type": "text",
        "text": "23"
      }
    ]
  },
  {
    "record_id": "recFNRH5QF",
    "value": [
      {
        "type": "text",
        "text": "456"
      }
    ]
  }
]
```
