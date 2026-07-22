---
document_id: '6965379541104263173'
directory_id: '6907567266536308737'
title: getStorageSync
full_path: /uYjL24iN/uATOx4CM5EjLwkTM
breadcrumb:
- Client API
- Web app/Gadget API
- Cache
- getStorageSync
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:33Z
source_url: https://open.larksuite.com/document/uYjL24iN/uATOx4CM5EjLwkTM
---

# getStorageSync(string key)

获取本地缓存数据。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="vh" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **x** | **x** | **x** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| key | string | 是 |  | 键名<br>**示例值**：stateKey<br>**最小长度**：`1`  字符 |



## 输出
返回值：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| data | string&#124;object&#124;number&#124;boolean&#124;object[]&#124;string[]&#124;number[]&#124;boolean[]&#124;undefined&#124;null | 键名对应的数据 |


## 示例代码

```js
try {
    let result = tt.getStorageSync("name");
    console.log(`getStorageSync success: ${JSON.stringify(result)}`);
} catch (error) {
    console.log(`getStorageSync fail: ${JSON.stringify(error)}`);
}
```

返回值示例：
```json
"小王"
```
