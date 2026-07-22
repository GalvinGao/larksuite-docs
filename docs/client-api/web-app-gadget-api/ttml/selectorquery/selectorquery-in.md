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

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createSelectorQuery/createSelectorQuery" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |


## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| component | object | 是 |  | 自定义组件实例 |


## 输出

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| selectorQuery | object | [SelectorQuery](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectorquery) 实例 |


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



