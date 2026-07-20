---
document_id: '7073692582770327558'
directory_id: '7073451436034129925'
title: UpdateManager.onUpdateFailed
full_path: /uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/onupdatefailed
breadcrumb:
- Client API
- Web app/Gadget API
- Update
- UpdateManager
- UpdateManager.onUpdateFailed
document_type: GuideDocumentType
updated_at: 2022-03-11T04:21:03Z
source_url: https://open.larksuite.com/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/onupdatefailed
---

# UpdateManager.onUpdateFailed(function callback)

监听小程序更新失败事件。小程序有新版本，客户端主动触发下载（无需开发者触发），下载失败（可能是网络原因等）后回调


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
      <md-td> <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> 
</md-td>
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
                callback
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                小程序更新失败事件的回调函数
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出
无


## 示例代码
```js
const updateManager = tt.getUpdateManager();
updateManager.onUpdateFailed(function(res) {
    console.log(`onUpdateFailed:${JSON.stringify(res)}`);
});
updateManager.triggerCheckUpdate();
```
回调函数返回对象示例：
```json
onUpdateFailed:{}
```

