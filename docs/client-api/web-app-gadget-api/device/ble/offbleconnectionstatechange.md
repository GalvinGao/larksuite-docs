---
document_id: '7329718153025896453'
directory_id: '6907567266537799681'
title: offBLEConnectionStateChange
full_path: /uYjL24iN/uMTOxYjLzkTM24yM5EjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- BLE
- offBLEConnectionStateChange
document_type: GuideDocumentType
updated_at: 2024-01-31T08:44:25Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTOxYjLzkTM24yM5EjN
---

# offBLEConnectionStateChange(function callback)

取消监听蓝牙低功耗连接状态的改变事件

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
无


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
tt.offBLEConnectionStateChange(() => {
  console.log('offBLEConnectionStateChange');
});
```





**注意**

为防止多次注册事件监听导致一次事件多次回调，建议每次调用on方法监听事件之前，先调用off方法，关闭之前的事件监听。

**说明**

不传递 callback 值，则会移除监听所有的事件监听回调。示例代码如下：

```js
tt.offBLEConnectionStateChange();
```

传递 callback 值，只移除对应的 callback 事件。示例代码如下：

```js
tt.offBLEConnectionStateChange(this.callback);
```
