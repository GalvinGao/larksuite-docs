---
document_id: '7260081693314793478'
directory_id: '7258197168736665606'
title: table.addRecord
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_addrecord
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.addRecord
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_addrecord
---

# table.addRecord
添加一行记录。

## 权限要求
:::html
<md-alert type="warn">
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>

:::


## 输入
```
addRecord(recordValues)
```

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| recordValues | {<br>&nbsp;&nbsp;fields: {<br>&nbsp;&nbsp;&nbsp;&nbsp;[fieldId: string]: [IOpenCellValue](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iopencellvalue)<br>&nbsp;&nbsp;}<br>} | 否 | 需要设置的字段id和它的值 |



## 输出
Promise字符串: 新增的记录的记录id。
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const field = await table.getFieldByName('多行文本'); // 选择某个多行文本字段

const res = await table.addRecord({
  fields: {
    [field.id]: [ // 多行文本对应的值的格式
      {
        type: 'text',
        text: '123'
      }
    ]
  }
})

```


### 返回示例
res:
```js
'rec0nzYabv'
```
