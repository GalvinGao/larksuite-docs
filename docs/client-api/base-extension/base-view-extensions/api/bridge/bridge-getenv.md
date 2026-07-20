---
document_id: '7281229133445988357'
directory_id: '7258197168736632838'
title: bridge.getEnv
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/bridge/bridge_getenv
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- Bridge
- bridge.getEnv
document_type: GuideDocumentType
updated_at: 2023-09-22T02:26:40Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/bridge/bridge_getenv
---

# bridge.getEnv
获取当前文档环境信息（如品牌信息）。



## 输出
Promise 对象，指明当前品牌信息
```js
{
    product: 'lark' | 'Lark';
}
```
## 示例代码
### 调用示例

```js
const res = await bitable.bridge.getEnv();
```


### 返回示例
res:
```js
{
    product: 'lark';
}
```
