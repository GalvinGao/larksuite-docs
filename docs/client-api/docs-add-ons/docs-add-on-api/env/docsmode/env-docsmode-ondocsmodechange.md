---
document_id: '7270779700749090821'
directory_id: '7270719284443496453'
title: Env.DocsMode.onDocsModeChange
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Env.DocsMode.onDocsModeChange
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Env
- DocsMode
- Env.DocsMode.onDocsModeChange
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Env.DocsMode.onDocsModeChange
---

# Env.DocsMode.onDocsModeChange
监听文档模式变化，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

| **名称**  | **数据类型**              | **是否必填** | **描述**    |
| ------- | --------------------- | -------- | --------- |
| handler | DocsModeChangeHandler | 是        | 文档模式变化监听器 |

### DarkModeChangeHandler

```js
type DocsModeChangeHandler = (docsMode: DOCS_MODE) => void;
```
| **key** | **value** | **描述**                |
| ------- | --------- | --------------------- |
| EDITING | EDITING   | 编辑态                   |
| NONE    | NONE      | 无配置                   |
| READING | READING   | 阅读态                   |
| UNKNOWN | UNKNOWN   | 未知：模板注入失败 或 未刷新切换新的文档 |

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docsModeChangeHandler = (docsMode: DOCS_MODE) => {
        console.log('debug', docsMode);
    };
useEffect(() => {
    docMiniApp.Env.DocsMode.onDocsModeChange(docsModeChangeHandler);
    return () => {
        docMiniApp.Env.DocsMode.offDocsModeChange(docsModeChangeHandler);
    };
});
```

### 返回示例

无
