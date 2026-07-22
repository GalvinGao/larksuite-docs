---
document_id: '7329718153025830917'
directory_id: '6907567266540797954'
title: offBluetoothDeviceFound
full_path: /uYjL24iN/uEDOxYjLxgTM24SM4EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Bluetooth
- offBluetoothDeviceFound
document_type: GuideDocumentType
updated_at: 2024-01-31T08:43:57Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEDOxYjLxgTM24SM4EjN
---

# offBluetoothDeviceFound(function callback)

取消监听寻找到新设备的事件

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.25.0+</md-version> | <md-version>V3.25.0+</md-version> | **X** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.16.0+</md-version> | <md-version>V5.16.0+</md-version> | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |



## 输出
无


## 示例代码

```js
tt.offBluetoothDeviceFound(this.callback);
```


