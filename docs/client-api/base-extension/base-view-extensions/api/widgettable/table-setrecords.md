---
document_id: '7281229133446053893'
directory_id: '7258197168736665606'
title: table.setRecords
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_setrecords
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.setRecords
document_type: GuideDocumentType
updated_at: 2023-09-22T02:26:44Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_setrecords
---

# table.setRecords
批量修改记录，最多5000条。

## 权限要求
:::html
<md-alert type="warn">
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>

:::


## 输入
```
setRecords(records)
```

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| records | IRecord[] | 是 | 需要修改的记录 |

  
  ```js
  interface IRecord {
    recordId: string;
    fields: {
        [fieldId: string]: IOpenCellValue;
    };
}
  ```


## 输出
Promise字符串数组：修改记录的id（同参数顺序）。
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const recordIds = await table.getRecordIdList(); // 获取所有行id
const field = await table.getFieldByName('多行文本'); // 选择多行文本字段

const res = await table.setRecords([
    {
        recordId: recordIds[0],
        fields: {
         [field.id]: [
            {
                    type: 'text',
                    text: '1'
                }
             ]
        }
    },
    {
        recordId: recordIds[1],
        fields: {
         [field.id]: [
            {
                    type: 'text',
                    text: '2'
                }
             ]
        }
    }
])
console.log(res) // ['recxxx', 'recyyy']
```
