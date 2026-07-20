---
document_id: '6971043590354075653'
directory_id: '6907567266541879298'
title: offWindowResize
full_path: /uYjL24iN/uIDO3UjLygzN14iM4cTN
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Window
- offWindowResize
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:59Z
source_url: https://open.larksuite.com/document/uYjL24iN/uIDO3UjLygzN14iM4cTN
---

# offWindowResize(function callback)

取消监听窗口尺寸变化事件
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
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td><md-version>V3.13.0+</md-version></md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td> 
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
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

::: note
如果不传递，不会取消所有windowResize事件
:::

## 输出
无


## 代码示例

```js
const callback = function (res) {
  const size = res.size;
  const windowWidth = size.windowWidth;
  const windowHeight = size.windowHeight; 
  console.log(JSON.stringify(size))
};
 
tt.offWindowResize(callback);
```
 
