---
document_id: '7329718153026027525'
directory_id: '6907567266540797954'
title: getBluetoothDevices
full_path: /uYjL24iN/uQDOxYjL0gTM24CN4EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Bluetooth
- getBluetoothDevices
document_type: GuideDocumentType
updated_at: 2024-01-31T08:43:53Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQDOxYjL0gTM24CN4EjN
---

# getBluetoothDevices(Object object)

getBluetoothDevices(Object object) 用于获取在蓝牙模块生效期间所有已发现的蓝牙设备，包括已经和本机处于连接状态的设备。

## 注意事项

- 该接口获取到的设备列表为蓝牙模块生效期间所有搜索到的蓝牙设备。若在蓝牙模块使用流程结束后未及时调用 `tt.closeBluetoothAdapter` 接口释放资源，则调用该接口时可能返回历史蓝牙使用流程中搜索到的设备，该类设备可能已经不在用户身边，无法连接。

- 蓝牙设备在被搜索到时，系统返回的 `name` 字段一般为广播包中的 `LocalName` 字段中的设备名称，而如果与蓝牙设备建立连接，系统返回的 `name` 字段会改为从蓝牙设备上获取到的 `GattName`。若需要动态改变设备名称并展示，建议使用 `localName` 字段。



## 支持说明

该接口支持小程序和网页应用调用，对应的客户端版本支持情况如下所示。

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">应用能力</md-th>
      <md-th style="width: 20%;">Android</md-th>
       <md-th style="width: 20%;">iOS</md-th>
      <md-th style="width: 20%;">PC</md-th>
      <md-th style="width: 20%;">预览效果</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>小程序</md-td>
      <md-td><md-version>V3.25+</md-version></md-td>
      <md-td><md-version>V3.25+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44+</md-version></md-td>
      <md-td><md-version>V3.44+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> </md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入

该接口继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性。


## 输出

该接口继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success` 返回对象的扩展属性如下所示。

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 25%;">
                名称
            </md-th>
            <md-th style="width: 20%;">
                数据类型
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                devices
            </md-td>
            <md-td>
                object[]
            </md-td>
            <md-td>
                uuid 对应的已连接的蓝牙设备列表。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    name
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                蓝牙设备名称，某些设备可能没有。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    deviceId
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用于区分设备的 id。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    RSSI
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                当前蓝牙设备的信号强度。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    advertisData
                </md-text>
            </md-td>
            <md-td>
                ArrayBuffer
            </md-td>
            <md-td>
                当前蓝牙设备的广播数据段中的 ManufacturerData 数据段。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    advertisServiceUUIDs
                </md-text>
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                当前蓝牙设备的广播数据段中的 ServiceUUIDs 数据段。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    localName
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                当前蓝牙设备的广播数据段中的 LocalName 数据段。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    serviceData
                </md-text>
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                当前蓝牙设备的广播数据段中的 ServiceData 数据段。
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 示例代码

调用示例：

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
tt.getBluetoothDevices({,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getBluetoothDevices fail: ${JSON.stringify(res)}`);
    }
});
```
`success`返回对象示例：
```json
{
    "devices": [
        {
            "RSSI": -97,
            "advertisData": {},
            "advertisServiceUUIDs": [],
            "deviceId": "FF:24:79:5D:6D:0C",
            "serviceData": {}
        }]
}
```
`fail`返回对象示例：
```json
{
    "errMsg": "getBluetoothDevices:fail not init",
    "errCode": 10000
}
```



## 错误码

`fail` 返回对象中可能包含 errCode 属性和 errno 属性，均代表错误码。

**errCode 错误码**

通用错误码可参见[蓝牙 API 错误码](/document/uYjL24iN/uYzNxYjL2cTM24iN3EjN)。

**errno 错误码**

关于 Errno 错误码的详细说明以及通用错误码列表，可参见[Errno 错误码](/document/uYjL24iN/uAjMuAjMuAjM/errno)。
