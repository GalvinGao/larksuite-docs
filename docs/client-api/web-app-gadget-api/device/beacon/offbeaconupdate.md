---
document_id: '7330180313525698566'
directory_id: '7073451436034113541'
title: offBeaconUpdate
full_path: /uYjL24iN/uQTOuQTOuQTO/ibeacon/offbeaconupdate
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Beacon
- offBeaconUpdate
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:44Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/offbeaconupdate
---

# offBeaconUpdate(function callback)

取消监听 Beacon 设备更新事件

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V4.6.0+</md-version> | <md-version>V4.6.0+</md-version> | **X** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V4.6.0+</md-version> | <md-version>V4.6.0+</md-version> | **X** | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |



## 输出
无


## 示例代码

```js
const callback = (res) => {
	console.log(res);
};
tt.onBeaconUpdate(callback);
tt.offBeaconUpdate(callback);
```


