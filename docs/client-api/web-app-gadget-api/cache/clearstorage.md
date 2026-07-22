---
document_id: '6965379543683973126'
directory_id: '6907567266536308737'
title: clearStorage
full_path: /uYjL24iN/uUTOx4SN5EjL1kTM
breadcrumb:
- Client API
- Web app/Gadget API
- Cache
- clearStorage
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:39Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUTOx4SN5EjL1kTM
---

# clearStorage(Object object)

清理**全部**本地缓存数据。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="vh" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **x** | **x** | **x** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性


## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码

```js
tt.clearStorage({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`clearStorage fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "clearStorage:ok"
}
```
