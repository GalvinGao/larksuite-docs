---
document_id: '7260082411118657541'
directory_id: '7258197168736665606'
title: table.onRecordAdd
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_onrecordadd
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.onRecordAdd
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:37Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_onrecordadd
---

# table.onRecordAdd
监听记录添加事件，将返回一个取消监听函数。

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
onRecordAdd((event) => {})
```

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| event | <code>{<br>  "name": string,<br>  "type": "publish",<br>  "data": string[],<br>}</code> | 否 | data为新增的recordId数组 |



## 输出
取消监听的函数。
## 示例代码

```js
    const selection = await bitable.base.getSelection();
    const table = await bitable.base.getTableById(selection.tableId);

    const field = await table.getFieldByName('多行文本') // 根据字段名获取多行文本类型的字段
    const off = table.onRecordAdd((event) => { // 监听字段增加事件。
        off();
        console.log('event:', event);
    })

    table.addRecord({ // 新增一行记录。
        fields: {
            [field.id]: [{
                type: 'text',
                text: '新增的记录'
            }]
        }
    })
```

