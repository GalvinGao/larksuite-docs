---
document_id: '6965379541104295941'
directory_id: '6907567266540765186'
title: createSelectorQuery
full_path: /uYjL24iN/uYjN24iN2YjL2YjN
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- createSelectorQuery
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:48Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYjN24iN2YjL2YjN
---

# createSelectorQuery()

返回一个 [SelectorQuery](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectorquery) 对象实例。

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

返回值：[SelectorQuery](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectorquery)，该对象的方法列表参见下表：
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 50%;">方法</md-th>
      <md-th style="width: 50%;">介绍</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
    <md-td>[SelectorQuery.in](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/in)</md-td>
    <md-td>将选择器的选取范围更改为自定义组件 component 内（初始时，选择器仅选取页面范围的节点，不会选取任何自定义组件中的节点）</md-td>
  </md-tr>
<md-tr>
    <md-td>[SelectorQuery.select](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/select)</md-td>
    <md-td>在当前页面下选择第一个匹配选择器 selector 的节点，返回一个 NodesRef 对象实例，可以用于获取节点信息。  selector 类似于 CSS 的选择器，其中移动端只支持 ID 选择器。</md-td>
  </md-tr>
<md-tr>
    <md-td>[SelectorQuery.selectAll](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectall)</md-td>
    <md-td>在当前页面下选择匹配选择器 selector 的所有节点，返回一个 NodesRef 对象实例，可以用于获取节点信息。 selector 类似于 CSS 的选择器，同 select。</md-td>
  </md-tr>
<md-tr>
    <md-td>[SelectorQuery.selectViewport](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectviewport)</md-td>
    <md-td>选择显示区域。可用于获取显示区域的尺寸、滚动位置等信息。</md-td>
  </md-tr>
<md-tr>
    <md-td>[SelectorQuery.exec](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/exec)</md-td>
    <md-td>执行所有的请求。请求结果按请求次序构成数组，在 callback 的第一个参数中返回。</md-td>
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
const query = tt.createSelectorQuery()
query.select('#the-id').boundingClientRect()
query.selectViewport().scrollOffset()
query.exec(function(res){
  res[0].top       // #the-id 节点的上边界坐标
  res[1].scrollTop // 显示区域的竖直滚动位置
})
```
