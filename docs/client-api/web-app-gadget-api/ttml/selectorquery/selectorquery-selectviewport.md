---
document_id: '7073692582769360902'
directory_id: '7073451436034080773'
title: SelectorQuery.selectViewport
full_path: /uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectviewport
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- SelectorQuery
- SelectorQuery.selectViewport
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:51Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectviewport
---

# SelectorQuery.selectViewport()

选择显示区域。可用于获取显示区域的尺寸、滚动位置等信息。

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
      <md-td><md-version>V2.2.0+</md-version></md-td>
      <md-td><md-version>V2.2.0+</md-version></md-td>
      <md-td><md-version>V2.2.0+</md-version></md-td>
       <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createSelectorQuery/createSelectorQuery" fontSize="14">预览</md-preview-app>
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
无

## 输出
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

            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                NodesRef
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                [NodesRef](/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/nodesref) 对象
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createSelectorQuery/createSelectorQuery" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::
```js

const query = tt.createSelectorQuery();
query.select('#the-id').boundingClientRect();
const nodesRef = query.selectViewport();
nodesRef.scrollOffset();
query.exec(function(res){
  res[0].top       // #the-id 节点的上边界坐标
  res[1].scrollTop // 显示区域的竖直滚动位置
});
```



