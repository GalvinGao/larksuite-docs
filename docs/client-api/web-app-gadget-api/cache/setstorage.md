---
document_id: '6965379543684661254'
directory_id: '6907567266536308737'
title: setStorage
full_path: /uYjL24iN/uETOx4SM5EjLxkTM
breadcrumb:
- Client API
- Web app/Gadget API
- Cache
- setStorage
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:24Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOx4SM5EjLxkTM
---

# setStorage(Object object)

以「键值对」的形式设置本地缓存数据。

:::html
<md-alert type="tip">
注意事项：
- 单个 key 允许存储的最大数据长度为 **1MB**，所有数据存储上限为 **10MB**，同时也受到用户设备存储空间、缓存清理等机制的限制，可能会导致信息丢失，因此请不要将重要数据存放在本地数据缓存。
</md-alert>
:::

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
| data | string&#124;object&#124;number&#124;boolean&#124;object[]&#124;string[]&#124;number[]&#124;boolean[]&#124;undefined&#124;null | 否 | undefined | 键名对应的数据<br>**示例值**：小王 |



## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码

```js
tt.setStorage({
    key: "name",
    data: "小王",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`setStorage fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "setStorage:ok"
}
```
