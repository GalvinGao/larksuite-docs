---
document_id: '7330180313525731334'
directory_id: '6907567266537799681'
title: notifyBLECharacteristicValueChange
full_path: /uYjL24iN/uETOxYjLxkTM24SM5EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- BLE
- notifyBLECharacteristicValueChange
document_type: GuideDocumentType
updated_at: 2024-06-17T12:53:06Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOxYjLxkTM24SM5EjN
---

# notifyBLECharacteristicValueChange(Object object)

监听特征值数据变化
:::html
<md-alert type="tip">
注意事项：
- 订阅操作成功后需要设备主动更新特征值的 value，才会触发 [tt.onBLECharacteristicValueChange](/document/uYjL24iN/uQTOxYjL0kTM24CN5EjN) 。
- 订阅方式效率比较高，推荐使用订阅代替 [read](/document/uYjL24iN/uYTOxYjL2kTM24iN5EjN) 方式。
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
              蓝牙特征值对应 service 的 uuid

            </md-td>
        </md-tr>
              <md-tr>
            <md-td>
                characteristicId
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
               蓝牙特征值的 uuid

            </md-td>
        </md-tr>
                 <md-tr>
            <md-td>
                descriptorId
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
               notify 的 descriptor 的 uuid （只有android 会用到，非必填）

            </md-td>
        </md-tr>
                       <md-tr>
            <md-td>
                state
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
               是否启用notify或indicate

            </md-td>
        </md-tr>
       <md-tr>
            <md-td>
                type
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>notify</md-td>
            <md-td>
               设置特征订阅类型，有效值为：
- notify
- indicate
              
**注意**：
- Lark 版本为 [V7.19.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上时，支持设置该字段。        
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
tt.notifyBLECharacteristicValueChange({
    deviceId: "F16DF2DF-0406-5EAA-E7F7-8BE962C0B88D",
    serviceId: "D0611E78-BBB4-4591-A5F7-487910AE4355",
    characteristicId: "00002A19-0000-1000-8001-00805F8B34FB",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`notifyBLECharacteristicValueChange fail: ${JSON.stringify(res)}`);
    }
});

```

`success`返回对象示例：

```json
{
	"errMsg": "notifyBLECharacteristicValueChange:ok"
}
``` 
`fail`返回对象示例：
```json
{
    "errMsg": "notifyBLECharacteristicValueChange:fail not init",
    "errCode": 10000
}
```


## 错误码

`fail`返回对象中会包含[errorCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码。

错误码可参见 [蓝牙 API 错误码](/document/uYjL24iN/uYzNxYjL2cTM24iN3EjN)
