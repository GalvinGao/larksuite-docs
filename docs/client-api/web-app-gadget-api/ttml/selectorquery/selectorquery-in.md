---
document_id: '7073692582770098182'
directory_id: '7073451436034080773'
title: SelectorQuery.in
full_path: /uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/in
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- SelectorQuery
- SelectorQuery.in
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:51Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/in
---

# SelectorQuery.in(object component)

将选择器的选取范围更改为自定义组件 `component` 内（初始时，选择器仅选取页面范围的节点，不会选取任何自定义组件中的节点）。

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
                component
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                自定义组件实例
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

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
                selectorQuery
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                [SelectorQuery](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectorquery) 实例
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
Component({
  queryMultipleNodes (){
    const query = tt.createSelectorQuery().in(this)
    query.select('#the-id').boundingClientRect(function(res){
      res.top // 这个组件内 #the-id 节点的上边界坐标
    }).exec()
  }
})
```



