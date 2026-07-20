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
      <md-td><md-version>V7.3+</md-version></md-td>
      <md-td><md-version>V7.3+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>/</md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th style="width: 10%;">
                必填
            </md-th>
            <md-th style="width: 10%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                services
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                蓝牙设备主 service 的 uuid 列表

**示例值**：['0000181a-0000-1000-8000-00805f9b34fb']
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 输出

`success`返回对象的扩展属性：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
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
                搜索到的设备列表
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
                蓝牙设备名称，某些设备可能没有
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
                用于区分设备的 id
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


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
