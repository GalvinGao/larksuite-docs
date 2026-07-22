---
document_id: '7329718153025945605'
directory_id: '6907567266540797954'
title: offBluetoothAdapterStateChange
full_path: /uYjL24iN/uIDOxYjLygTM24iM4EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Bluetooth
- offBluetoothAdapterStateChange
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:00Z
source_url: https://open.larksuite.com/document/uYjL24iN/uIDOxYjLygTM24iM4EjN
---

# offBluetoothAdapterStateChange(function callback)

取消监听蓝牙适配器状态变化事件

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.25.0+</md-version> | <md-version>V3.25.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.16.0+</md-version> | <md-version>V5.16.0+</md-version> | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


## 输出
无


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
const callback = (res) => {
	console.log(res);
};
tt.onBluetoothAdapterStateChange(callback);
tt.offBluetoothAdapterStateChange(callback);
```


