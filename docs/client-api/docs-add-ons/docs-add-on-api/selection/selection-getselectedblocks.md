---
document_id: '7270779605447081990'
directory_id: '7270719284443365381'
title: Selection.getSelectedBlocks
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Selection.getSelectedBlocks
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Selection
- Selection.getSelectedBlocks
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Selection.getSelectedBlocks
---

# Selection.getSelectedBlocks
获取指定文档当前选中的 Blocks，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

指定文档的引用
| **名称** | **数据类型**                                                                                                                                       | **是否必填** | **描述**  |
| ------ | ---------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ------- |
| docRef | [DocumentRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/DocumentRef) | 是        | 指定文档的引用 |
  

## 输出

异步返回指定文档当前选中的 Block数组
| **名称**         | **数据类型**                                                                                                          | **描述**             |
| -------------- | ----------------------------------------------------------------------------------------------------------------- | ------------------ |
| selectedBlocks | [BlockSnapshot](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockSnapshot) | 指定文档当前选中的 Block的数组 |
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
const selectedBlocks = await DocMiniApp.Selection.getSelectedBlocks(docRef);
console.log('debug',selectedBlocks);
```

### 返回示例

```json
[
  {
    "id": 6,
    "type": "text",
    "children": [],
    "childSnapshots": [],
    "parent": 1,
    "childIndex": 1,
    "data":
    {
      "plain_text": "asdasda",
      "text":
      {
        "elements":
          [
            {
              "text_run":
              {
                "content": "asdasda",
                "style":
                {
                  "inline_code": false, "bold": false, "italic": false, "underline": false, "strikethrough": false
                }
              }
            }
          ]
      }
    },
    "recordId": "recordId",
    "ref":
    {
      "docRef":
      {
        "docToken": "docx token"
      },
      "blockId": 6
    }
  }
]
```
