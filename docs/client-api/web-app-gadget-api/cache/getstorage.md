---
document_id: '6965379543683350534'
directory_id: '6907567266536308737'
title: getStorage
full_path: /uYjL24iN/ukDOx4SO4EjL5gTM
breadcrumb:
- Client API
- Web app/Gadget API
- Cache
- getStorage
document_type: GuideDocumentType
updated_at: 2022-03-28T02:19:41Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDOx4SO4EjL5gTM
---

# getStorage(Object object)

获取本地缓存数据.


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/storage/storage" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名词 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| key | string | 是 |  | 键名<br>**示例值**: name<br>**最小长度**: `1`  字符 |



## 输出

`success`返回对象的扩展属性:

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| data | string&#124;object&#124;number&#124;boolean&#124;object[]&#124;string[]&#124;number[]&#124;boolean[]&#124;undefined&#124;null | 键名对应的数据 |


##  示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/storage/storage" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.getStorage({
    key: "name",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getStorage fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例:
```json
{
    "data": "Xiao Wang",
    "errMsg": "getStorage:ok"
}
``` 


# getStorage(Object object)

获取本地缓存数据.


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/storage/storage" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名词 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| key | string | 是 |  | 键名<br>**示例值**: name<br>**最小长度**: `1`  字符 |



## 输出

`success`返回对象的扩展属性:

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| data | string&#124;object&#124;number&#124;boolean&#124;object[]&#124;string[]&#124;number[]&#124;boolean[]&#124;undefined&#124;null | 键名对应的数据 |


##  示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/storage/storage" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.getStorage({
    key: "name",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getStorage fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例:
```json
{
    "data": "Xiao Wang",
    "errMsg": "getStorage:ok"
}
``` 


