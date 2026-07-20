---
document_id: '7330180313525649414'
directory_id: '6907567266537799681'
title: setBLEMTU
full_path: /uYjL24iN/uMTMyYjLzEjM24yMxIjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- BLE
- setBLEMTU
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:04Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTMyYjLzEjM24yMxIjN
---

# setBLEMTU(Object object)

设置蓝牙最大传输单元。需在 tt.connectBLEDevice调用成功后调用，mtu 设置范围 (22,512)。


:::html
<md-alert type="tip">
注意事项：
- 仅安卓操作系统5.1以上有效
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
      <md-td><md-version>V3.26+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>


    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V7.3+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>/</md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th style="width: 10%;">
                必填
            </md-th>
            <md-th style="width: 10%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                deviceId
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
               用于区分设备的 id
              
**示例值**：'FF:24:79:5D:6D:0C'
            </md-td>
        </md-tr>
              <md-tr>
            <md-td>
                mtu
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
               最大传输单元(22,512) 区间内，单位 bytes

            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::



## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


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
tt.setBLEMTU({
    // 这里的 deviceId 为的 getBluetoothDevices 或 onBluetoothDeviceFound 接口中获取到的 device's id
    deviceId: "",
    mtu: 200,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`setBLEMTU fail: ${JSON.stringify(res)}`);
    }
});

```

`success`返回对象示例：

```json
{
	"errMsg": "setBLEMTU:ok"
}
``` 


## 错误码
`fail`返回对象中会包含[errorCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码。

通用错误码可参见 [蓝牙 API 错误码](/document/uYjL24iN/uYzNxYjL2cTM24iN3EjN)



