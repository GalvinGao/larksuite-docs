---
document_id: '7073692582768885766'
directory_id: '7073451436034146309'
title: NodesRef.scrollOffset
full_path: /uYjL24iN/uUjN24SN2YjL1YjN/nodesref/scrolloffset
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- NodesRef
- NodesRef.scrollOffset
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:51Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/scrolloffset
---

# NodesRef.scrollOffset(function callback)

添加节点的滚动位置查询请求。以像素为单位。节点必须是 `scroll-view` 或者 `viewport`，返回 `nodesRef` 对应的 `SelectorQuery`。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createSelectorQuery/createSelectorQuery" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| id | string | 节点的 id |
| dataset | object | 节点的 dataset |
| scrollLeft | number | 节点的水平滚动位置 |
| scrollTop | number | 节点的垂直滚动位置 |


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
Page({
  getScrollOffset: function(){
    tt.createSelectorQuery().selectViewport().scrollOffset(function(res){
      res.id      // 节点的ID
      res.dataset // 节点的dataset
      res.scrollLeft // 节点的水平滚动位置
      res.scrollTop  // 节点的竖直滚动位置
    }).exec()
  }
});
```
