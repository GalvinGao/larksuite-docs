---
document_id: '7270779605449998342'
directory_id: '7270719451987738630'
title: Block.insertBlock
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.insertBlock
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Block
- Block.insertBlock
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Block.insertBlock
---

# Block.insertBlock
在父 Block 的指定位置插入一个新 Block，返回新插入的 Block 的快照信息，该方法为异步调用。

## 注意事项
:::html
<md-alert type="error">以下 Block 暂不支持插入 BITABLE、CELL、CHAT_CARD、DIAGRAM、FILE、GRID、GRID_COLUMN、IFRAME、IMAGE、ISV、MINDNOTE、PAGE、SHEET、TABLE、VIEW</md-alert>
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
| blockSnapshot  | [CreateBlockSnapshot](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/CreateBlockSnapshot) | 是        | 插入的 Block 快照数据。以下 Block 暂不支持插入：见注意事项 |
  

## 输出

异步返回新插入的 Block 的快照信息，它是一个[BlockSnapshot](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockSnapshot)
  

## 示例代码

### 调用示例

```js
    const content = 'text';
    const DocMiniApp = new BlockitClient().initAPI();
    const docRef = await DocMiniApp.getActiveDocumentRef();
    const rootBlockRef = await DocMiniApp.Document.getRootBlock(docRef);
    // 若是不清楚想设置的block都需要哪些信息，可以先创建好block后选中再调用getSelectedBlocks打印一下相关信息
    const text = await DocMiniApp.Block.insertBlock({ blockId: rootBlockRef.id, docRef }, 0, {
        type: BlockType.TEXT,
            data: {
              text: {
                elements: [
                  {
                    text_run: {
                      content,
                      style: {
                          bold: true
                      }
                    }
                  }
                ]
              },
              plain_text: content
            }
     });
    console.log('debug',text);
```

### 返回示例

```json
{
  "id": 8,
    "type": "text",
      "children": [],
        "childSnapshots": [],
          "parent": 1,
            "childIndex": 0,
              "data":
  {
    "plain_text": "text",
      "text":
    {
      "elements":
      [
        {
          "text_run":
          {
            "content": "text",
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
    "blockId": 8
  }
}s
```
