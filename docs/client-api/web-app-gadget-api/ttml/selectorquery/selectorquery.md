---
document_id: '7073693024735297541'
directory_id: '7073451436034080773'
title: SelectorQuery
full_path: /uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectorquery
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- SelectorQuery
- SelectorQuery
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:51Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectorquery
---

# SelectorQuery


SelectorQuery 查找节点信息的对象，可通过 [createSelectorQuery](/document/uYjL24iN/uYjN24iN2YjL2YjN)获取。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createSelectorQuery/createSelectorQuery" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 方法

### [SelectorQuery.in](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/in)

将选择器的选取范围更改为自定义组件 `component` 内（初始时，选择器仅选取页面范围的节点，不会选取任何自定义组件中的节点）

### [SelectorQuery.select](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/select)
在当前页面下选择第一个匹配选择器 `selector` 的节点，返回一个 `NodesRef` 对象实例，可以用于获取节点信息。 `selector` 类似于 CSS 的选择器，其中移动端只支持 ID 选择器。


### [SelectorQuery.selectAll](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectall)

在当前页面下选择匹配选择器 `selector` 的所有节点，返回一个 `NodesRef` 对象实例，可以用于获取节点信息。 `selector` 类似于 CSS 的选择器，同 [select](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/select)。

### [SelectorQuery.selectViewport](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectviewport)

选择显示区域。可用于获取显示区域的尺寸、滚动位置等信息。

### [SelectorQuery.exec](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/exec)

执行所有的请求。请求结果按请求次序构成数组，在 `callback` 的第一个参数中返回。



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
