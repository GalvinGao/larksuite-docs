---
document_id: '7329718153025994757'
directory_id: '6907567266540797954'
title: startBluetoothDevicesDiscovery
full_path: /uYjL24iN/uUzNxYjL1cTM24SN3EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Bluetooth
- startBluetoothDevicesDiscovery
document_type: GuideDocumentType
updated_at: 2024-01-31T08:43:42Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUzNxYjL1cTM24SN3EjN
---

# startBluetoothDevicesDiscovery(Object object)


调用 startBluetoothDevicesDiscovery(Object object) 可以开始搜寻附近的蓝牙外围设备。

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

该接口继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性如下所示。

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">
                名称
            </md-th>
            <md-th style="width: 15%;">
                数据类型
            </md-th>
            <md-th style="width: 15%;">
                是否必填
            </md-th>
            <md-th style="width: 15%;">
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
            <md-td>\-</md-td>
            <md-td>
                要搜索的蓝牙设备主服务（service）的 uuid 列表。
- 某些蓝牙设备会广播自己的主服务 uuid。如果设置此参数，则只搜索广播包有对应 uuid 的主服务的蓝牙设备。
- 建议主要通过该参数过滤掉周边不需要处理的其他蓝牙设备。

**示例值**：['0000181a-0000-1000-8000-00805f9b34fb']
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                allowDuplicatesKey
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>\-</md-td>
            <md-td>
                是否允许重复上报同一设备。取值：
- true：允许
- false：不允许

如果允许重复上报，则 `tt.onBlueToothDeviceFound` 方法会多次上报同一设备，但是 RSSI 值会有不同。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                interval
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>\-</md-td>
            <md-td>
                上报设备的间隔。
- `0` 表示找到新设备立即上报。
- 其他数值根据传入的间隔上报，单位：ms。例如，传入 `3` 表示每间隔 3 ms 上报一次。
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 输出

该接口继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性。

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
tt.startBluetoothDevicesDiscovery({
    "services": [],
    "allowDuplicatesKey": true,
    "interval": 0,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`startBluetoothDevicesDiscovery fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "startBluetoothDevicesDiscovery:ok"
}
```
`fail`返回对象示例：
```json
{
    "errMsg": "startBluetoothDevicesDiscovery:fail not init",
    "errCode": 10000
}
```

## 错误码

`fail` 返回对象中可能包含 errCode 属性和 errno 属性，均代表错误码。

**errCode 错误码**

通用错误码可参见[蓝牙 API 错误码](/document/uYjL24iN/uYzNxYjL2cTM24iN3EjN)。

**errno 错误码**

关于 Errno 错误码的详细说明以及通用错误码列表，可参见[Errno 错误码](/document/uYjL24iN/uAjMuAjMuAjM/errno)。
