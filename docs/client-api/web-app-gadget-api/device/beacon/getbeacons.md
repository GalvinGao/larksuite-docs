---
document_id: '7330180313525682182'
directory_id: '7073451436034113541'
title: getBeacons
full_path: /uYjL24iN/uQTOuQTOuQTO/ibeacon/getbeacons
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Beacon
- getBeacons
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:40Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/getbeacons
---

# getBeacons(Object object)

getBeacons(Object object) 用于获取所有已搜索到的 Beacon 设备。

## 注意事项

调用该接口前，需要先调用 [startBeaconDiscovery](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/startbeacondiscovery)。


## 支持说明

该接口支持小程序和网页应用调用，对应的客户端版本支持情况如下所示。

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V4.6.0+</md-version> | <md-version>V4.6.0+</md-version> | **X** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V4.6.0+</md-version> | <md-version>V4.6.0+</md-version> | **X** | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入

该接口继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性。


## 输出

该接口继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success` 返回对象的扩展属性如下所示。

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| beacons | [BeaconInfo[]](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/ibeaconinfo) | Beacon 设备列表。 |
| &emsp;<br><span style="color: #8F959E">∟</span><br>&nbsp;<br><md-text type="field-name">uuid</md-text> | string | Beacon 设备广播的 uuid。 |
| &emsp;<br><span style="color: #8F959E">∟</span><br>&nbsp;<br><md-text type="field-name">major</md-text> | number | Beacon 设备的主 id。<br><md-alert type="tip" icon="none"><br>**注意**：历史存在 Android 中 `major` 以及 `minor` 字段为 string 类型的问题，该问题在Lark V5.26.0 版本已修复。</md-alert> |
| &emsp;<br><span style="color: #8F959E">∟</span><br>&nbsp;<br><md-text type="field-name">minor</md-text> | number | Beacon 设备的次 id。<br><md-alert type="tip" icon="none"><br>**注意**：历史存在 Android 中 `major` 以及 `minor` 字段为 string 类型的问题，该问题在Lark V5.26.0 版本已修复。</md-alert> |
| &emsp;<br><span style="color: #8F959E">∟</span><br>&nbsp;<br><md-text type="field-name">proximity</md-text> | number | 表示设备距离的枚举值。可能值：<br>- `0`：无效<br>- `1`：非常近<br>- `2`：近<br>- `3`：远 |
| &emsp;<br><span style="color: #8F959E">∟</span><br>&nbsp;<br><md-text type="field-name">accuracy</md-text> | number | Beacon 设备的距离。 |
| &emsp;<br><span style="color: #8F959E">∟</span><br>&nbsp;<br><md-text type="field-name">rssi</md-text> | number | 表示设备的信号强度。 |


## 示例代码

调用示例：

```js
tt.startBeaconDiscovery({
    uuids: [
        "fda50693-a4e2-4fb1-afcf-c6eb07647825"
    ],
    ignoreBluetoothAvailable: true,
    success(res) {
      tt.getBeacons({ 
    	success(res) {
      	  console.log(JSON.stringify(res));
    	},
    	fail(res) {
      	  console.log(`getBeacons fail: ${JSON.stringify(res)}`);
    	}
	    });
    },
    fail(res) {
      console.log(`startBeaconDiscovery fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "beacons": [
        {
            "accuracy": 10.467388920465797,
            "major": 11054,
            "minor": 11389,
            "proximity": 3,
            "rssi": -81,
            "uuid": "FDA50693-A4E2-4FB1-AFCF-C6EB07647825"
        }
    ],
    "errMsg": "getBeacons:ok"
}
```


## 错误码

`fail` 返回对象中可能包含 errCode 属性和 errno 属性，均代表错误码。具体错误码可查阅 [Beacon API 错误码](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/ibeacon-api-error-code) 或 [Errno 错误码](/document/uYjL24iN/uAjMuAjMuAjM/errno)。

