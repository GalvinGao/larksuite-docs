---
document_id: '7330180313525633030'
directory_id: '6907567266540797954'
title: onBluetoothDeviceFound
full_path: /uYjL24iN/ukzNxYjL5cTM24SO3EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Bluetooth
- onBluetoothDeviceFound
document_type: GuideDocumentType
updated_at: 2024-01-31T08:43:57Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukzNxYjL5cTM24SO3EjN
---

# onBluetoothDeviceFound(function callback)

监听寻找到新设备的事件

:::html
<md-alert type="tip">
注意事项：
- 若在 tt.onBluetoothDeviceFound 回调了某个设备，则此设备会添加到 tt.getBluetoothDevices 接口获取到的数组中。
- 安卓下部分机型需要有位置权限才能搜索到设备，需留意是否开启了位置权限
</md-alert>
:::

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
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td> 
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V5.16+</md-version></md-td>
      <md-td><md-version>V5.16+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>/</md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::



## 输入
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">名称</md-th>
      <md-th style="width: 18%;">数据类型</md-th>
       <md-th style="width: 10%;">必填</md-th>
      <md-th style="width: 10%;">默认值</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>

    
   <md-tr>
      <md-td>callback</md-td>
      <md-td>function</md-td>
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>该事件的回调函数</md-td>

   </md-tr>  
    

    
</md-tbody>
</md-table>
:::

## 输出
回调函数返回对象的属性：
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
                新搜索到的设备列表
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
                当前蓝牙设备的信号强度
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
                当前蓝牙设备的广播数据段中的 ServiceUUIDs 数据段
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
                当前蓝牙设备的广播数据段中的 LocalName 数据段
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
                当前蓝牙设备的广播数据段中的 ServiceData 数据段
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 示例代码

```js
// ArrayBuffer转16进制字符串示例
function ab2hex(buffer) {
	var hexArr = Array.prototype.map.call(
    new Uint8Array(buffer),
    function(bit) {
      return ('00' + bit.toString(16)).slice(-2)
    }
  )
  return hexArr.join('');
}

tt.onBluetoothDeviceFound(function(res) {
  var devices = res.devices;
  console.log('new device list has founded')
  console.dir(devices)
  console.log(ab2hex(devices[0].advertisData))
})
```

回调函数返回对象示例：

```json
{
  "devices": [
    {
      "RSSI": -88,
      "advertisServiceUUIDs": [
        "0000181a-0000-1000-8000-00805f9b34fb"
      ],
      "deviceId": "E5:66:9F:82:46:61",
      "localName": "LYWSD02",
      "name": "LYWSD02",
      "serviceData": {
        "0000fe95-0000-1000-8000-00805f9b34fb": {}
      }
    }
  ]
}
``` 



## 错误码

`fail`返回对象中会包含[errorCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码。

通用错误码可参见 [蓝牙 API 错误码](/document/uYjL24iN/uYzNxYjL2cTM24iN3EjN)
