---
document_id: '7330180313525698566'
directory_id: '7073451436034113541'
title: offBeaconUpdate
full_path: /uYjL24iN/uQTOuQTOuQTO/ibeacon/offbeaconupdate
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Beacon
- offBeaconUpdate
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:44Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/offbeaconupdate
---

# offBeaconUpdate(function callback)

取消监听 Beacon 设备更新事件

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
无


## 示例代码

```js
const callback = (res) => {
	console.log(res);
};
tt.onBeaconUpdate(callback);
tt.offBeaconUpdate(callback);
```


