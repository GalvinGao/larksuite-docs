---
document_id: '7260081693314547718'
directory_id: '7258197168736632838'
title: bridge.setData
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/bridge_setdata
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- Bridge
- bridge.setData
document_type: GuideDocumentType
updated_at: 2024-03-07T08:44:09Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/bridge_setdata
---

# bridge.setData
在当前base中存储一些内容。

> 在调试阶段使用该接口需要将 `@lark-opdev/block-bitable-webpack-utils` 依赖版本升级至 `0.1.6` 及以上。


## 示例代码
### 调用示例

```js
await bitable.bridge.setData({
    a:123
});

const res = await bitable.bridge.getData();
console.log(res)
```


### 返回示例
res:
```js
{a: 123}
```
