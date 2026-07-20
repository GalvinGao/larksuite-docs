---
document_id: '7329718153025830917'
directory_id: '6907567266540797954'
title: offBluetoothDeviceFound
full_path: /uYjL24iN/uEDOxYjLxgTM24SM4EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Bluetooth
- offBluetoothDeviceFound
document_type: GuideDocumentType
updated_at: 2024-01-31T08:43:57Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEDOxYjLxgTM24SM4EjN
---

# offBluetoothDeviceFound(function callback)

取消监听寻找到新设备的事件

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
      <md-td><md-version>V3.25.0+</md-version></md-td>
      <md-td><md-version>V3.25.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td> 
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V5.16.0+</md-version></md-td>
      <md-td><md-version>V5.16.0+</md-version></md-td>
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
无


## 示例代码

```js
tt.offBluetoothDeviceFound(this.callback);
```


