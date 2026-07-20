---
document_id: '7073691561008300037'
directory_id: '7073451436034097157'
title: onMemoryWarning
full_path: /uYjL24iN/uQTOuQTOuQTO/performance/onmemorywarning
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- performance
- onMemoryWarning
document_type: GuideDocumentType
updated_at: 2022-03-11T04:17:37Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQTOuQTOuQTO/performance/onmemorywarning
---

# onMemoryWarning(function callback)

监听内存不足的告警事件。当手机内存占用过高时，触发回调函数。该事件不会杀掉小程序, 建议开发者可以在接受到告警后释放不必要的资源。 在Android平台下有告警等级划分。


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
      <md-td><md-version>V5.4.0+</md-version></md-td>
      <md-td><md-version>V5.4.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td>

</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**/**</md-td>

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
                level
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                对应系统内存告警等级宏（Level）定义，数值越高，告警等级越高。

**可选值**：
- `5`：TRIM_MEMORY_RUNNING_MODERATE
- `10`：TRIM_MEMORY_RUNNING_LOW
- `15`：TRIM_MEMORY_RUNNING_CRITICAL
<md-alert type="tip" icon="none">
仅 Android 端返回该字段
</md-alert>
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">

  <div style="display: flex">
  </div>
</div> 
:::

```js
tt.onMemoryWarning(function () {
  console.log("onMemoryWarning");
});
```

回调函数返回对象示例（仅Android）：

```json
{"level":"5"}
