---
document_id: '7330180313525567494'
directory_id: '7073451436034113541'
title: stopBeaconDiscovery
full_path: /uYjL24iN/uQTOuQTOuQTO/ibeacon/stopbeacondiscovery
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Beacon
- stopBeaconDiscovery
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:36Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/stopbeacondiscovery
---

# stopBeaconDiscovery(Object object)

停止搜索附近的 Beacon 设备

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

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性



## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


## 示例代码


```js
tt.startBeaconDiscovery({
    uuids: [
        "fda50693-a4e2-4fb1-afcf-c6eb07647825"
    ],
    ignoreBluetoothAvailable: true,
    success(res) {
      tt.stopBeaconDiscovery({ 
    	success(res) {
      	  console.log(JSON.stringify(res));
    	},
    	fail(res) {
      	  console.log(`stopBeaconDiscovery fail: ${JSON.stringify(res)}`);
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
    "errMsg": "stopBeaconDiscovery:ok"
}
```



## 错误码
`fail`返回对象中会包含[errorCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码。具体错误码列表参见：[Beacon API错误码](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/ibeacon-api-error-code)
