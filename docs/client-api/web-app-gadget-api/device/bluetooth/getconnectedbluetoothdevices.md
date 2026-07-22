---
document_id: '7330180313525616646'
directory_id: '6907567266540797954'
title: getConnectedBluetoothDevices
full_path: /uYjL24iN/uMDOxYjLzgTM24yM4EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Bluetooth
- getConnectedBluetoothDevices
document_type: GuideDocumentType
updated_at: 2024-01-31T08:43:49Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMDOxYjLzgTM24yM4EjN
---

# getConnectedBluetoothDevices(Object object)


根据 uuid 获取处于已连接状态的设备。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.25+</md-version> | <md-version>V3.25+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V7.3+</md-version> | <md-version>V7.3+</md-version> | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| services | string[] | 否 |  | 蓝牙设备主 service 的 uuid 列表<br>**示例值**：['0000181a-0000-1000-8000-00805f9b34fb'] |



## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| devices | object[] | 搜索到的设备列表 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>name<br></md-text> | string | 蓝牙设备名称，某些设备可能没有 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>deviceId<br></md-text> | string | 用于区分设备的 id |



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
tt.getConnectedBluetoothDevices({
    "services": [],
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getConnectedBluetoothDevices fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "devices": [{
      "name":"xxxx",
      "deviceId":"E5:66:9F:82:46:61"
    }],
    "errMsg": "getConnectedBluetoothDevices:ok"
}
```
`fail`返回对象示例：
```json
{
    "errMsg": "getConnectedBluetoothDevices:fail not init",
    "errCode": 10000
}
```

## 错误码

`fail`返回对象中会包含[errorCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码。

通用错误码可参见 [蓝牙 API 错误码](/document/uYjL24iN/uYzNxYjL2cTM24iN3EjN)
