---
document_id: '7270779700749058053'
directory_id: '7270719284443299845'
title: Interaction.setData
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Interaction.setData
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Interaction
- Interaction.setData
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Interaction.setData
---

# Interaction.setData
修改当前云文档小组件的 Interaction 数据，该方法为异步调用。
## 主要事项
建议使用applyTransaction

## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | - 正文小组件<br>- 全屏视图<br>- 模态框视图<br>- 弹窗视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

| **名称**     | **数据类型**                                                                                                                                                | **是否必填** | **描述**             |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ------------------ |
| changesets | [InteractionChangeset](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/InteractionChangeset) | 是        | 要变更的Interaction 数据 |

## 输出

变更后的Interaction数据
| **名称**          | **数据类型**        | **是否必填** | **描述**        |
| --------------- | --------------- | -------- | ------------- |
| InteractionData | InteractionData | 是        | 变更后的Record 数据 |
  

## 示例代码

### 调用示例

- insert 类型，在某个路径下插入新的数据
```js
const newData = await DocMiniApp.Interaction.setData([
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
const newData =await DocMiniApp.Interaction.setData([
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
const newData = await DocMiniApp.Interaction.setData([
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
