---
document_id: '7329718153025880069'
directory_id: '6907567266537799681'
title: getBLEDeviceServices
full_path: /uYjL24iN/uATOxYjLwkTM24CM5EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- BLE
- getBLEDeviceServices
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:25Z
source_url: https://open.larksuite.com/document/uYjL24iN/uATOxYjLwkTM24CM5EjN
---

# getBLEDeviceServices(Object object)

低功耗蓝牙获取设备服务


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.25+</md-version> | <md-version>V3.25+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V7.3+</md-version> | <md-version>V7.3+</md-version> | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| deviceId | string | 是 |  | 蓝牙设备 ID<br>**示例值**：E5:66:9F:82:46:61 |


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| services | object[] | 已发现的设备服务列表。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>isPrimary<br></md-text> | boolean | 该服务是否为主服务。是主服务则为 true ，反之为 false 。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>serviceId<br></md-text> | string | 蓝牙设备特征值对应服务的 uuid。 |



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
tt.getBLEDeviceServices({
      deviceId: 'E5:66:9F:82:46:61',
      success: (res) => {
  		console.log(JSON.stringify(res));
      },
      fail: (error) => {
        console.log('getBLEDeviceServices fail:${JSON.stringify(res)}');
      }
    });
```
`success`返回对象示例：

```json
{
  "errMsg": "getBLEDeviceServices:ok ok",
  "innerMsg": "ok",
  "services": [
    {
      "isPrimary": true,
      "serviceId": "0000fe95-0000-1000-8000-00805f9b34fb"
    }
  ]
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
