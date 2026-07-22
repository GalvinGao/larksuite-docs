---
document_id: '7329718153026043909'
directory_id: '6907567266537799681'
title: onBLECharacteristicValueChange
full_path: /uYjL24iN/uQTOxYjL0kTM24CN5EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- BLE
- onBLECharacteristicValueChange
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:29Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQTOxYjL0kTM24CN5EjN
---

# onBLECharacteristicValueChange(function callback)
监听特征值数据变化

:::html
<md-alert type="tip">
注意事项：
- 为防止多次注册事件监听导致一次事件多次回调，建议每次调用on方法监听事件之前，先调用off方法，关闭之前的事件监听。

</md-alert>
:::

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.25+</md-version> | <md-version>V3.25+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V7.3+</md-version> | <md-version>V7.3+</md-version> | **X** | / |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| deviceId | string | 蓝牙设备 ID，参考 device 对象。 |
| serviceId | string | 蓝牙特征值对应 service 的 UUID。 |
| characteristicId | string | 蓝牙特征值的 UUID。 |
| value | hex string | 特征值最新的 16 进制值。 |




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
tt.onBLECharacteristicValueChange(function(res) {
    console.log(JSON.stringify(res));
});
```

返回对象示例：

```json
{
  "deviceId": "E5:66:9F:82:46:61",
  "serviceId": "xxx",
  "characteristicId": "xxx",
  "value": "xxx"
}
``` 


