---
document_id: '7270779700748795909'
directory_id: '7270719284443463685'
title: Record.setRecord
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Record.setRecord
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Record
- Record.setRecord
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Record.setRecord
---

# Record.setRecord
修改当前云文档小组件的 Record 数据，该方法为异步调用。

## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可写 | - 正文小组件<br>- 全屏视图<br>- 模态框视图<br>- 弹窗视图 | - PC<br>- 移动端 | 编辑模式 |



## 输入

Record 变更数据
| **名称**     | **数据类型**                                                                                                                                           | **是否必填** | **描述**        |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ------------- |
| changesets | [RecordChangeset](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/RecordChangeset) | 是        | 要变更的Record 数据 |
  

## 输出

变更后的Record数据
| **名称** | **数据类型** | **是否必填** | **描述**        |
| ------ | -------- | -------- | ------------- |
| data   | any      | 是        | 变更后的Record 数据 |
  

## 示例代码

### 调用示例

- insert 类型，在某个路径下插入新的数据
```js
const newData = await DocMiniApp.Record.setRecord([
    {
      type: 'insert',
      data: {
        path: [],
        value: { a:1 }
      }
    }
  ]);
console.log('debug', newData);//{ a:1 }
```
- remove 类型，将某个路径下的数据删除
```js
const newData =await DocMiniApp.Record.setRecord([
    {
      type: 'remove',
      data: {
        path: [],
      }
    }
  ]);
console.log('debug', newData);//{}
```
- replace 类型，将某个路径下的数据替换成新数据
```js
const newData = await DocMiniApp.Record.setRecord([
    {
      type: 'replace',
      data: {
        path: [], 
        value: { a:2 }
      }
    }
  ]);
console.log('debug', newData);//{ a:2 }
```
