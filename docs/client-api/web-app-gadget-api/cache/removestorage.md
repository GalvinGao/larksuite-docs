---
document_id: '6965379543683956742'
directory_id: '6907567266536308737'
title: removeStorage
full_path: /uYjL24iN/uMTOx4yM5EjLzkTM
breadcrumb:
- Client API
- Web app/Gadget API
- Cache
- removeStorage
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:36Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTOx4yM5EjLzkTM
---

# removeStorage(Object object)

删除本地缓存数据。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="vh" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **x** | **x** | **x** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| key | string | 是 |  | 键名<br>**示例值**：name<br>**最小长度**：`1`  字符 |



## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码

```js
tt.removeStorage({
    key: "name",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`removeStorage fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "removeStorage:ok"
}
```
