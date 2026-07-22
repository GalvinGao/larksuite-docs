---
document_id: '7329718153026011141'
directory_id: '6907567266540797954'
title: stopBluetoothDevicesDiscovery
full_path: /uYjL24iN/uczNxYjL3cTM24yN3EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Bluetooth
- stopBluetoothDevicesDiscovery
document_type: GuideDocumentType
updated_at: 2024-01-31T08:43:46Z
source_url: https://open.larksuite.com/document/uYjL24iN/uczNxYjL3cTM24yN3EjN
---

# stopBluetoothDevicesDiscovery(Object object)


停止搜寻附近的蓝牙外围设备。若已经找到需要的蓝牙设备并不需要继续搜索时，建议调用该接口停止蓝牙搜索。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.25+</md-version> | <md-version>V3.25+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44+</md-version> | <md-version>V3.44+</md-version> | **X** | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性

## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

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
tt.stopBluetoothDevicesDiscovery({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`stopBluetoothDevicesDiscovery fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "stopBluetoothDevicesDiscovery:ok"
}
```
`fail`返回对象示例：
```json
{
    "errMsg": "stopBluetoothDevicesDiscovery:fail not init",
    "errCode": 10000
}
```

## 错误码

`fail`返回对象中会包含[errorCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码。

通用错误码可参见 [蓝牙 API 错误码](/document/uYjL24iN/uYzNxYjL2cTM24iN3EjN)
