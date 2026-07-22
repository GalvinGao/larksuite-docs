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

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.25+</md-version> | <md-version>V3.25+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V7.3+</md-version> | <md-version>V7.3+</md-version> | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| deviceId | string | 是 |  | 蓝牙设备 ID。<br>**示例值**：E5:66:9F:82:46:61 |


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
