---
document_id: '7329718153026043909'
directory_id: '6907567266537799681'
title: onBLECharacteristicValueChange
full_path: /uYjL24iN/uQTOxYjL0kTM24CN5EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- BLE
- onBLECharacteristicValueChange
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:29Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQTOxYjL0kTM24CN5EjN
---

# onBLECharacteristicValueChange(function callback)
监听特征值数据变化

:::html
<md-alert type="tip">
注意事项：
- 为防止多次注册事件监听导致一次事件多次回调，建议每次调用on方法监听事件之前，先调用off方法，关闭之前的事件监听。

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
                deviceId
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                蓝牙设备 ID，参考 device 对象。
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
                蓝牙特征值对应 service 的 UUID。
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
                蓝牙特征值的 UUID。
            </md-td>
        </md-tr>
                    <md-tr>
            <md-td>
                value
            </md-td>
            <md-td>
                hex string
            </md-td>
            <md-td>
                特征值最新的 16 进制值。
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::



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
tt.onBLECharacteristicValueChange(function(res) {
    console.log(JSON.stringify(res));
});
```

返回对象示例：

```json
{
  "deviceId": "E5:66:9F:82:46:61",
  "serviceId": "xxx",
  "characteristicId": "xxx",
  "value": "xxx"
}
``` 


