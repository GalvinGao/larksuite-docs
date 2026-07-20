---
document_id: '6965379543684399110'
directory_id: '6907567266540847106'
title: API 概述
full_path: /uYjL24iN/uADOy4CM4IjLwgjM
breadcrumb:
- Client API
- Web app/Gadget API
- API Overview
document_type: GuideDocumentType
updated_at: 2022-03-11T04:12:17Z
source_url: https://open.larksuite.com/document/uYjL24iN/uADOy4CM4IjLwgjM
---

# 概述

小程序提供了多种 API 来暴露客户端能力，如获取用户信息、本地存储等功能，分为两种：同步、异步。其中异步 API 通过回调返回结果，同步 API 直接返回结果。

:::note 
**PC端 API 支持版本：3.8.0+**
:::
## 代码示例

```js
tt.API({
    success (res) {
        console.log(`API 调用成功 ${res}`);
    },
    fail (res) {
        console.log(`API 调用失败`);
    }
});
```

```js
try {
    var res = tt.APISync();
    console.log(`API 调用成功 ${res}`);
} catch (error) {
    console.log(`API 调用失败`);
}
```
