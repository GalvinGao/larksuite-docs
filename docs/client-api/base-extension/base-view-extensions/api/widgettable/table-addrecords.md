---
document_id: '7281229133446004741'
directory_id: '7258197168736665606'
title: table.addRecords
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_addrecords
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.addRecords
document_type: GuideDocumentType
updated_at: 2023-09-22T02:26:44Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_addrecords
---

# table.addRecords
批量新增记录，最多5000条。

## 权限要求
:::html
<md-alert type="warn">
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>

:::


## 输入
```
addRecords(recordValueList)
```

:::html
<md-table>
  <colgroup>
    <col style="width: 150px;">
    <col style="width: 350PX">
    <col style="width: auto">
    <col style="width: auto">
  </colgroup>
	<md-thead> 
      <md-tr>
      	<md-th>名称</md-th>
        <md-th>数据类型</md-th>
        <md-th>是否必填</md-th>
        <md-th>描述</md-th>
      </md-tr>
  </md-thead> 
  	<md-tbody>
      <md-tr>
      	<md-td>recordValueList</md-td>
        <md-td>{<br>
    &nbsp;&nbsp;fields: {<br>
        &nbsp;&nbsp;&nbsp;&nbsp;[fieldId: string]: [IOpenCellValue]([IOpenCellValue](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iopencellvalue));<br>
    &nbsp;&nbsp;};<br>
}[]</md-td>
        <md-td>是</md-td>
        <md-td>需要新增的记录</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
Promise字符串数组：新增记录的id（同参数顺序）。
## 示例代码
### 调用示例

```js
import { bitable, IOpenSegmentType } from '@base-open/web-api';

const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const field = await table.getFieldByName('多行文本'); // 选择某个多行文本字段
const field2 = await table.getFieldByName('单选'); // 选择某个单选字段
const field2Meta = await field2.getMeta() // 获取单选字段的选项信息等.
const field3 = await table.getFieldByName('多选'); // 选择某个多选字段
const field3Meta = await field3.getMeta() // 获取多选字段的选项信息等.

const res = await table.addRecords([
    {// 添加1行
      fields: {
        [field.id]: [ // 添加不同类型的字段的时候，这个值的类型是不一样的,这里以添加多行文本对应的值的格式 为例,如果是其它类型的字段可以参考这里
          {
            type: IOpenSegmentType.Text,
            text: '1'
          }
        ]
      }
    },
    {// 添加第2行
      fields: {
        [field2.id]: { // 注意field2是单选字段,添加记录的时候，同时给单选字段设置一个值，就需要用单选字段的值格式
          id: field2Meta.property.options[0].id, //选第一个选项
        }
      }
    },
    {// 添加第3行
      fields: {
        [field3.id]: [// 注意field3是多选字段
            { // 添加记录的时候，同时给多选字段设置一个值，就需要用多选字段的值格式
              id: field3Meta.property.options[0].id, //选第一个选项
            },
            {
              id: field3Meta.property.options[1].id, //选第2个选项
            }
        ]
      }
    },
]);
console.log(res) // ['recxxx', 'recyyyy'....]    新增的记录的 id 列表
```
