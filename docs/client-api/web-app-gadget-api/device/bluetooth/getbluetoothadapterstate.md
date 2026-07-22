---
document_id: '7330180313525600262'
directory_id: '6907567266540797954'
title: getBluetoothAdapterState
full_path: /uYjL24iN/uUDOxYjL1gTM24SN4EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Bluetooth
- getBluetoothAdapterState
document_type: GuideDocumentType
updated_at: 2024-01-31T08:43:38Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUDOxYjL1gTM24SN4EjN
---

# getBluetoothAdapterState(Object object)

获取本机蓝牙适配器状态。

:::html
<md-alert type="tip">

</md-alert>
:::


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.25+</md-version> | <md-version>V3.25+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44+</md-version> | <md-version>V3.44+</md-version> | **X** | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| discovering | boolean | 是否正在搜索设备 |
| available | boolean | 蓝牙适配器是否可用 |



## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.getBluetoothAdapterState({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getBluetoothAdapterState fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "available": true,
    "discovering": false,
    "errMsg": "getBluetoothAdapterState:ok"
}
```
`fail`返回对象示例：
```json
{
    "errMsg": "getBluetoothAdapterState:fail not init",
    "errCode": 10000
}
```

## 错误码
`fail`返回对象中会包含[errorCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码。

通用错误码可参见 [蓝牙 API 错误码](/document/uYjL24iN/uYzNxYjL2cTM24iN3EjN)
