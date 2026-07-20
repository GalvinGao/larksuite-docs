---
document_id: '7330180313525764102'
directory_id: '6907567266537799681'
title: disconnectBLEDevice
full_path: /uYjL24iN/ugDOxYjL4gTM24CO4EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- BLE
- disconnectBLEDevice
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:22Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDOxYjL4gTM24CO4EjN
---

# disconnectBLEDevice(Object object)

断开设备连接


:::html
<md-alert type="tip">
注意事项：
- 蓝牙连接随时可能断开，建议监听 tt.onBLEConnectionStateChange 回调事件，当蓝牙设备断开时按需执行重连操作。
- 若对未接的设备或已断开连接的设备调用数据读写操作的接口，会返回10006错误，详见错误码，建议进行重连操作。
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
                deviceId
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                蓝牙设备 ID。

**示例值**：E5:66:9F:82:46:61
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


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
tt.disconnectBLEDevice({
      deviceId: 'E5:66:9F:82:46:61',
      success: (res) => {
        console.log(JSON.stringify(res));
      },
      fail: (res) => {
        console.log('disconnectBLEDevice fail:${JSON.stringify(res)}');
      },
});
```

`success`返回对象示例：

```json
{
  "errMsg": "connectBLEDevice:ok"
}
``` 

## 错误码

`fail`返回对象中会包含[errorCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码。

通用错误码可参见 [蓝牙 API 错误码](/document/uYjL24iN/uYzNxYjL2cTM24iN3EjN)
