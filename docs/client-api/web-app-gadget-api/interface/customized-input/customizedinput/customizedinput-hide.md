---
document_id: '7073693024735772677'
directory_id: '7073450228347289605'
title: CustomizedInput.hide
full_path: /uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/hide
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Customized Input
- CustomizedInput
- CustomizedInput.hide
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:32Z
source_url: https://open.larksuite.com/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/hide
---

# CustomizedInput.hide()

隐藏输入框




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
      <md-td>**X**</md-td>
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
无



## 输出
无


## 示例代码



:::html
<md-alert type="tip">
展示输入框的代码参考：[customizedInput.show](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/show)
</md-alert>
:::

```js
//customizedInput通过tt.getCustomizedInput()方法获取，需要和show方法公用同一个实例
customizedInput.customizedInput.hide({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`customizedInput.hide fail: ${JSON.stringify(res)}`);
    }
});
```


