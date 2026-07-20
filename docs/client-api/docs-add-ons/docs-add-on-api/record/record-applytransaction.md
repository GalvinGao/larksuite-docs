---
document_id: '7270779605451423750'
directory_id: '7270719284443463685'
title: Record.applyTransaction
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Record.applyTransaction
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Record
- Record.applyTransaction
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Record.applyTransaction
---

# Record.applyTransaction
修改当前云文档小组件的 Record 数据，跟 setRecord 方法是等价的，只是通过 handler 的方式来产生 changesets，该方法为异步调用。
  
## 可用性说明
:::html
<md-table>
<md-thead>
<md-tr>
<md-th>权限要求</md-th>
<md-th>视图可用说明</md-th>
<md-th>平台可用</md-th>
<md-th>场景</md-th></md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>可写</md-td>
<md-td>- 正文小组件
- 全屏视图
- 模态框视图
- 弹窗视图</md-td>
<md-td>- PC
- 移动端</md-td>
<md-td>\-</md-td>
</md-tr>
</md-tbody>
</md-table>
:::


## 输入

Record 数据写入处理器
| **名称**  | **数据类型**         | **是否必填** | **描述**         |
| ------- | ---------------- | -------- | -------------- |
| handler | SetRecordHandler | 是        | Record 数据写入处理器 |
SetRecordHandler
```js
export type SetRecordHandler = (op: RecordOperation) => void;
```
| **名称** | **数据类型**                                                                                                         | **是否必填** | **描述**                |
| ------ | ---------------------------------------------------------------------------------------------------------------- | -------- | --------------------- |
| op     | [RecordOperation](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/RecordOperation) | 是        | Record Operation 操作对象 |

## 输出

1. 参数概要说明
1. 表格视图参数说明
  

## 示例代码

### 调用示例

```js
  await DocMiniApp.Record.applyTransaction(op => {
      op.replace(['aaa'], bbb);
    });
```
