---
document_id: '7330180313525813254'
directory_id: '6907567266540797954'
title: onBluetoothAdapterStateChange
full_path: /uYjL24iN/uADOxYjLwgTM24CM4EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Bluetooth
- onBluetoothAdapterStateChange
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:00Z
source_url: https://open.larksuite.com/document/uYjL24iN/uADOxYjLwgTM24CM4EjN
---

# onBluetoothAdapterStateChange(function callback)

监听蓝牙适配器状态变化事件


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.25+</md-version> | <md-version>V3.25+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.16+</md-version> | <md-version>V5.16+</md-version> | **X** | / |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| discovering | boolean | 蓝牙适配器是否处于搜索状态 |
| available | boolean | 蓝牙适配器是否可用 |


## 示例代码
  
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.onBluetoothAdapterStateChange(function (res) {
  console.log('adapterState changed, now is', res)
})
```

回调函数返回对象示例：

```json
{
  "discovering":true,
  "available",true
}
``` 



## 错误码
  
`fail`返回对象中会包含[errorCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码。
  
通用错误码可参见 [蓝牙 API 错误码](/document/uYjL24iN/uYzNxYjL2cTM24iN3EjN)
