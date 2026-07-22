---
document_id: '7330180313525846022'
directory_id: '7073451436034113541'
title: onBeaconUpdate
full_path: /uYjL24iN/uQTOuQTOuQTO/ibeacon/onbeaconupdate
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Beacon
- onBeaconUpdate
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:44Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/onbeaconupdate
---

# onBeaconUpdate(function callback)

监听 Beacon 设备更新事件，仅能注册一个监听

:::html
<md-alert type="tip">
注意事项：需要先调用[startBeaconDiscovery](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/startbeacondiscovery)。
</md-alert>
:::

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

回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| beacons | [BeaconInfo[]](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/ibeaconinfo) | Beacon 设备列表 |
| &emsp;<br><span style="color: #8F959E">∟</span><br>&nbsp;<br><md-text type="field-name">uuid</md-text> | string | Beacon 设备广播的 uuid |
| &emsp;<br><span style="color: #8F959E">∟</span><br>&nbsp;<br><md-text type="field-name">major</md-text> | number | Beacon 设备的主 id |
| &emsp;<br><span style="color: #8F959E">∟</span><br>&nbsp;<br><md-text type="field-name">minor</md-text> | number | Beacon 设备的次 id |
| &emsp;<br><span style="color: #8F959E">∟</span><br>&nbsp;<br><md-text type="field-name">proximity</md-text> | number | 表示设备距离的枚举值 |
| &emsp;<br><span style="color: #8F959E">∟</span><br>&nbsp;<br><md-text type="field-name">accuracy</md-text> | number | Beacon 设备的距离 |
| &emsp;<br><span style="color: #8F959E">∟</span><br>&nbsp;<br><md-text type="field-name">rssi</md-text> | number | 表示设备的信号强度 |



## 示例代码

```js
tt.startBeaconDiscovery({
    uuids: [
        "fda50693-a4e2-4fb1-afcf-c6eb07647825"
    ],
    ignoreBluetoothAvailable: true,
    success(res) {
      tt.onBeaconUpdate(function(res) {
        console.log(JSON.stringify(res));	
      });
    },
    fail(res) {
      console.log(`startBeaconDiscovery fail: ${JSON.stringify(res)}`);
    }
});
```

回调函数返回对象示例:
```json
{
    "beacons": [
        {
            "accuracy": 16.545591294123085,
            "major": 11054,
            "minor": 11389,
            "proximity": 3,
            "rssi": -86,
            "uuid": "FDA50693-A4E2-4FB1-AFCF-C6EB07647825"
        }
    ]
}
```


## 错误码
`fail`返回对象中会包含[errorCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码。具体错误码列表参见：[Beacon API错误码](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/ibeacon-api-error-code)


## 已知问题

- Android中`major`以及`minor`字段为string类型，在V5.26.0版本已修复。
