---
document_id: '7073692582769917958'
directory_id: '6907567266540748802'
title: onNetworkQualityChange
full_path: /uYjL24iN/uUTNx4SN1EjL1UTM/onnetworkqualitychange
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Network Status
- onNetworkQualityChange
document_type: GuideDocumentType
updated_at: 2022-03-11T04:16:55Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUTNx4SN1EjL1UTM/onnetworkqualitychange
---

# onNetworkQualityChange(function callback)

监听网络质量变化


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V4.9.0+</md-version> | <md-version>V4.9.0+</md-version> | <md-version>V5.1.0+</md-version> | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V4.9.0+</md-version> | <md-version>V4.9.0+</md-version> | <md-version>V5.1.0+</md-version> | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| networkQualityType | string | 网络分级类型，同 [getNetworkQualityType](/document/uYjL24iN/uUTNx4SN1EjL1UTM/getnetworkqualitytype) 中描述 |


## 示例代码



```js
tt.onNetworkQualityChange(function(res) {
    console.log(JSON.stringify(res));
});
```

回调函数返回对象示例：
```json
{
    "networkQualityType": "excellent"
}
```
 



