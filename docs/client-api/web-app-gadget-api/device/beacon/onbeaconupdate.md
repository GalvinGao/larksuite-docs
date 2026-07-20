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
      <md-td><md-version>V4.6.0+</md-version></md-td>
      <md-td><md-version>V4.6.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td> 
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V4.6.0+</md-version></md-td>
      <md-td><md-version>V4.6.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> </md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">名称</md-th>
      <md-th style="width: 18%;">数据类型</md-th>
       <md-th style="width: 10%;">必填</md-th>
      <md-th style="width: 10%;">默认值</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>

    
   <md-tr>
      <md-td>callback</md-td>
      <md-td>function</md-td>
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>该事件的回调函数</md-td>

   </md-tr>  
    

    
</md-tbody>
</md-table>
:::


## 输出

回调函数返回对象的属性：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">名称</md-th>
            <md-th style="width: 18%;">数据类型</md-th>
            <md-th>描述</md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>beacons</md-td>
            <md-td>[BeaconInfo[]](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/ibeaconinfo)</md-td>
            <md-td>Beacon 设备列表</md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">∟</span>
                &nbsp;
                <md-text type="field-name">uuid</md-text>
            </md-td>
            <md-td>string</md-td>
            <md-td>Beacon 设备广播的 uuid</md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">∟</span>
                &nbsp;
                <md-text type="field-name">major</md-text>
            </md-td>
            <md-td>number</md-td>
            <md-td>Beacon 设备的主 id</md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">∟</span>
                &nbsp;
                <md-text type="field-name">minor</md-text>
            </md-td>
            <md-td>number</md-td>
            <md-td>Beacon 设备的次 id</md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">∟</span>
                &nbsp;
                <md-text type="field-name">proximity</md-text>
            </md-td>
            <md-td>number</md-td>
            <md-td>表示设备距离的枚举值</md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">∟</span>
                &nbsp;
                <md-text type="field-name">accuracy</md-text>
            </md-td>
            <md-td>number</md-td>
            <md-td>Beacon 设备的距离</md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">∟</span>
                &nbsp;
                <md-text type="field-name">rssi</md-text>
            </md-td>
            <md-td>number</md-td>
            <md-td>表示设备的信号强度</md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


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
