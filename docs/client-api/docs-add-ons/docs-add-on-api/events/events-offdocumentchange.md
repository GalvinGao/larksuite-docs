---
document_id: '7270779605447000070'
directory_id: '7270719284443709445'
title: Events.offDocumentChange
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Events.offDocumentChange
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Events
- Events.offDocumentChange
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Events.offDocumentChange
---

# Events.offDocumentChange
取消监听文档内容变化，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

| **名称**  | **数据类型**                                                               | **是否必填** | **描述**  |
| ------- | ---------------------------------------------------------------------- | -------- | ------- |
| docRef  | [DocumentRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/DocumentRef) | 是        | 文档引用    |
| handler | DocumentChangeHandler                                                  | 是        | 文档变化监听器 |

### DocumentChangeHandler

```js
type DocumentChangeHandler = (event: DocumentChangeEvent) => void;
```

### DocumentChangeEvent

文档变化事件
| **名称**  | **数据类型**       | **是否必填** | **描述**  |
| ------- | -------------- | -------- | ------- |
| changes | BlockChange[] | 是        | 文档变更的内容 |

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const handleDocumentChange = (event: DocumentChangeEvent) => {
    console.log('debug', event);
];
useEffect(() => {
    const fn = async () => {
        const docRef = await DocMiniApp.getActiveDocumentRef();
        DocMiniApp.Events.onDocumentChange(docRef, handleDocumentChange);
    ];
    fn();
    return () => {
        docRef && DocMiniApp.Events.offDocumentChange(docRef, handleDocumentChange);
    };
});
```

### 返回示例

无
