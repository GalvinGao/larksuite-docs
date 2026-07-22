---
document_id: '7270779605450080262'
directory_id: '7270719284443627525'
title: Viewport.getViewportBlocks
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Viewport.getViewportBlocks
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Viewport
- Viewport.getViewportBlocks
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Viewport.getViewportBlocks
---

# Viewport.getViewportBlocks
获取视口中的所有 Block，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

无需传入参数。
  

## 输出

异步返回一个[BlockSnapshot](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockSnapshot)

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const inViewportBlocks = await DocMiniApp.Viewport.getViewportBlocks();
console.log('debug', inViewportBlocks);
```

### 返回示例

```json
[
    {
        "id": 2,
        "type": "isv",
        "children": [],
        "childSnapshots": [],
        "parent": 1,
        "childIndex": 0,
        "data": {
            "component_id": "",
            "component_type_id": "component type id",
            "data": {}
        },
        "recordId": "redord id",
        "ref": {
            "docRef": {
                "docToken": "doc token"
            },
            "blockId": 2
        }
    }
]
```
