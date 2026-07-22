---
document_id: '7329718153025814533'
directory_id: '7073451436034113541'
title: offBeaconServiceChange
full_path: /uYjL24iN/uQTOuQTOuQTO/ibeacon/offbeaconservicechange
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Beacon
- offBeaconServiceChange
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:47Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/offbeaconservicechange
---

# offBeaconServiceChange(function callback)

取消监听 Beacon 服务状态变化事件

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
tt.onBeaconServiceChange(callback);
tt.offBeaconServiceChange(callback);
```


