---
document_id: '7073691561007972357'
directory_id: '7073451436033966085'
title: RequestTask.abort
full_path: /uYjL24iN/ugDNugDNugDN/requesttask/abort
breadcrumb:
- Client API
- Web app/Gadget API
- Network
- initiating Request
- RequestTask
- RequestTask.abort
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:12Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDNugDNugDN/requesttask/abort
---

# RequestTask.abort()


中断请求任务



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
      <md-td>**✓**</md-td>
<md-td><md-preview-app type="vh" disable="true" fontSize="14">预览</md-preview-app></md-td> 
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


```js
try {
    const requestTask = tt.request({"url":"https://www.toutiao.com","data":{"noncestr":1637496519175},"header":{"content-type":"application/json"},"method":"GET","dataType":"json","responseType":"text"});
    requestTask.abort();
} catch (error) {
    console.log(`abort fail: ${JSON.stringify(error)}`);
}
```




