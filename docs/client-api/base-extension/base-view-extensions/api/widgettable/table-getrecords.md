---
document_id: '7281229133446021125'
directory_id: '7258197168736665606'
title: table.getRecords
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getrecords
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.getRecords
document_type: GuideDocumentType
updated_at: 2023-09-22T02:26:44Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getrecords
---

# table.getRecords
批量获取记录

## 权限要求
:::html
<md-alert type="warn">
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>

:::


## 输入
```
getRecords(param)
```

:::html
<md-table>
  <colgroup>
    <col style="width: 150px;">
    <col style="width: auto">
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
      	<md-td>param.pageSize</md-td>
        <md-td>number</md-td>
        <md-td>是</md-td>
        <md-td>最大获取数量5000</md-td>
      </md-tr>
      <md-tr>
      	<md-td>param.pageToken</md-td>
        <md-td>string</md-td>
        <md-td>否</md-td>
        <md-td>记录id,表示分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果</md-td>
      </md-tr>
      <md-tr>
      	<md-td>param.viewId</md-td>
        <md-td>string</md-td>
        <md-td>否</md-td>
        <md-td>获取指定视图的 record</md-td>
      </md-tr>
      
    </md-tbody>
</md-table>
:::


## 输出
Promise对象IGetRecordsResponse。
  
```js
interface IGetRecordsResponse {
    total: number;
    hasMore: boolean;
    records: IRecord[];
    pageToken?: string;
}
interface IRecord {
    recordId: string;
    fields: {
        [fieldId: string]: IOpenCellValue;
    };
}
```
  
  
## 示例代码
### 调用示例

```js
  const selection = await bitable.base.getSelection();
  const table = await bitable.base.getTableById(selection.tableId); // 获取当前数据表实例

  const res = await table.getRecords({pageSize:100})
  console.log(res)
```
