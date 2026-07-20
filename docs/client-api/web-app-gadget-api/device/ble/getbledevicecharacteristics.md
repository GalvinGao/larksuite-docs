---
document_id: '7329718153025961989'
directory_id: '6907567266537799681'
title: getBLEDeviceCharacteristics
full_path: /uYjL24iN/ukDOxYjL5gTM24SO4EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- BLE
- getBLEDeviceCharacteristics
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:15Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDOxYjL5gTM24SO4EjN
---

# getBLEDeviceCharacteristics(Object object)

获取读写特征

:::html
<md-alert type="tip">
注意事项：
- 建立连接后先执行 tt.getBLEDeviceServices 与 tt.getBLEDeviceCharacteristics 后再进行与蓝牙设备的数据交互。
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
      <md-td><md-version>V3.25+</md-version></md-td>
      <md-td><md-version>V3.25+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/bluetooth/bluetooth" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>


    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V7.3+</md-version></md-td>
      <md-td><md-version>V7.3+</md-version></md-td>
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
               蓝牙设备 ID，参考 device 对象
              
**示例值**：'FF:24:79:5D:6D:0C'

            </md-td>
        </md-tr>
      <md-tr>
            <md-td>
                serviceId
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
               蓝牙特征值对应 service 的 UUID

            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::



## 输出



`success`返回对象的扩展属性：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                characteristics
            </md-td>
            <md-td>
                object[]
            </md-td>
            <md-td>
                设备特征值列


            </md-td>
        </md-tr>
                    <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    characteristicId
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                蓝牙设备特征值的 UUID
            </md-td>
        </md-tr>
                    <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    serviceId
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                蓝牙设备特征值对应服务的 UUID
            </md-td>
        </md-tr>
                    <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    value
                </md-text>
            </md-td>
            <md-td>
                hex string
            </md-td>
            <md-td>
                蓝牙设备特征值对应的 16 进制值
            </md-td>
        </md-tr>
                         <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    properties
                </md-text>
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                该特征值支持的操作类型
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


**Properties 对象**

|名称                |类型      |描述       |
|------------------|--------|---------|
|read              |boolean |该特征值是否支持 read 操作|
|write             |boolean |该特征值是否支持 write 操作|
|notify            |boolean |该特征值是否支持 notify 操作|
|indicate          |boolean |该特征值是否支持 indicate 操作|





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
tt.getBLEDeviceCharacteristics({
    deviceId: "",
    serviceId: "",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getBLEDeviceCharacteristics fail: ${JSON.stringify(res)}`);
    }
});
```


`success`返回对象示例：

```json
{
  "characteristics": {
    "characteristicId": "xxx",
    "serviceId": "xxx",
    "value": "xxx",
    "properties": {
      "write": false,
      "notify": false,
      "read": true,
      "indicate": false
    }
  },
  "errMsg": "getBLEDeviceCharacteristics:ok",
}
``` 
`fail`返回对象示例：
```json
{
    "errMsg": "getBLEDeviceCharacteristics:fail not init",
    "errCode": 10000
}
```



## 错误码

`fail`返回对象中会包含[errorCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码。

错误码可参见 [蓝牙 API 错误码](/document/uYjL24iN/uYzNxYjL2cTM24iN3EjN)
