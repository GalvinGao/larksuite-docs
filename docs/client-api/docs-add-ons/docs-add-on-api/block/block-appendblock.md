---
document_id: '7270779700748894213'
directory_id: '7270719451987738630'
title: Block.appendBlock
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.appendBlock
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Block
- Block.appendBlock
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.appendBlock
---

# Block.appendBlock
在父 Block 的最后面插入一个新 Block，返回新插入的 Block 的快照信息，该方法为异步调用。

## 注意事项
:::html
<md-alert type="error">以下 Block 暂不支持插入：BITABLE、CELL、CHAT_CARD、DIAGRAM、FILE、GRID、GRID_COLUMN、IFRAME、IMAGE、ISV、MINDNOTE、PAGE、SHEET、TABLE、VIEW</md-alert>
:::
  
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
<md-td>所有视图</md-td>
<md-td>- PC
- 移动端</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

父 Block 的引用、父 Block 的位置以及插入的 Block 快照数据。
| **名称**         | **数据类型**                                                                                                                                               | **是否必填** | **描述**                                |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ | -------- | ------------------------------------- |
| parentBlockRef | [BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef)            | 是        | 父 Block 的引用                           |
| position       | number                                                                                                                                                 | 是        | 父 Block 的位置                           |
| blockSnapshot  | [CreateBlockSnapshot](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/CreateBlockSnapshot) | 是        | 插入的 Block 快照数据。以下 Block 暂不支持插入：见注意事项1 |
  

## 输出

异步返回返回新插入的 Block 的快照信息，它一个[BlockSnapshot](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockSnapshot)
  

## 示例代码

### 调用示例

```js
    const DocMiniApp = new BlockitClient().initAPI();
    const docRef = await DocMiniApp.getActiveDocumentRef();
    const blockSnap = await DocMiniApp.Document.getRootBlock(docRef);
    await DocMiniApp.Block.appendBlock(blockSnap.ref, {
      type: BlockType.TEXT,
      data: {
        text: {
          elements: [{
            text_run: {
              content: 'test',
              style: {
                underline: true
              }
            }
          }]
        },
        plain_text: 'test'
      }
    });
```

### 返回示例

```json
{
    "id": 10,
    "type": "text",
    "children": [],
    "childSnapshots": [],
    "parent": 1,
    "childIndex": 8,
    "data": {
        "text": {
            "elements": [
                {
                    "text_run": {
                        "content": "test",
                        "style": {
                            "inline_code": false,
                            "bold": false,
                            "italic": false,
                            "underline": true,
                            "strikethrough": false
                        }
                    }
                }
            ]
        },
        "plain_text": "test"
    },
    "recordId": "OY8qd0qAoogoA0x0QabccpJonbf",
    "ref": {
        "docRef": {
            "docToken": "Ob3gdQWR7oN9VKxGPc6czZmrnpd"
        },
        "blockId": 10
    }
}
```
