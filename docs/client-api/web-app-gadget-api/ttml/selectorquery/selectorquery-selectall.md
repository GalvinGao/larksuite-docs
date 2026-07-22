---
document_id: '7073691561008365573'
directory_id: '7073451436034080773'
title: SelectorQuery.selectAll
full_path: /uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectall
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- SelectorQuery
- SelectorQuery.selectAll
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:51Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectall
---

# 	SelectorQuery.selectAll(string selector)

在当前页面下选择匹配选择器 `selector` 的所有节点，返回一个 `NodesRef` 对象实例，可以用于获取节点信息。 `selector` 类似于 CSS 的选择器，同 `select` 。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createSelectorQuery/createSelectorQuery" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |


## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| selector | string | 是 |  | selector 名称 |


## 输出

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| NodesRef | object[] | [NodesRef](/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/nodesref) 对象列表 |



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
const nodesRef = tt.createSelectorQuery().selectAll('.class-name');
nodesRef.boundingClientRect((rects) => {
  rects.forEach((rect) => {
    rect.id      // 节点的ID
    rect.dataset // 节点的dataset
    rect.left    // 节点的左边界坐标
    rect.right   // 节点的右边界坐标
    rect.top     // 节点的上边界坐标
    rect.bottom  // 节点的下边界坐标
    rect.width   // 节点的宽度
    rect.height  // 节点的高度
  })
}).exec();
```



