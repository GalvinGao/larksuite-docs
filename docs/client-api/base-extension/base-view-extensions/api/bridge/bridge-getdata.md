---
document_id: '7260082411118575621'
directory_id: '7258197168736632838'
title: bridge.getData
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/bridge_getdata
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- Bridge
- bridge.getData
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:10Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/bridge_getdata
---

# bridge.getData
获取[bridge.setData](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/bridge_setdata)存储的内容。





## 输出
Promise 对象。
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
