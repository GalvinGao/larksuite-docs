---
document_id: '7073691561008201733'
directory_id: '7073451436034146309'
title: NodesRef
full_path: /uYjL24iN/uUjN24SN2YjL1YjN/nodesref/nodesref
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- NodesRef
- NodesRef
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:51Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/nodesref
---

# NodesRef


`ttml` 节点信息的对象。

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



## 方法

### [NodesRef.boundingClientRect](/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/boundingclientrect)

添加节点的布局位置的查询请求。相对于显示区域，以像素为单位。其功能类似于 DOM 的 `getBoundingClientRect`。返回 `nodesRef` 对应的 `SelectorQuery`。如果提供了 `callback` 回调函数，在执行 `selectQuery.exec` 方法后，节点信息会在 `callback` 中返回。

### [NodesRef.scrollOffset](/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/scrolloffset)
添加节点的滚动位置查询请求。以像素为单位。节点必须是 `scroll-view` 或者 `viewport`，返回 `nodesRef` 对应的 `SelectorQuery`。

### [NodesRef.fields](/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/fields)

获取节点的相关信息。需要获取的字段在`fields`中指定。返回值是`nodesRef`对应的 `selectorQuery`。


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
  getRect () {
    tt.createSelectorQuery().select('#the-id').boundingClientRect(function(rect){
      rect.id      // 节点的ID
      rect.dataset // 节点的dataset
      rect.left    // 节点的左边界坐标
      rect.right   // 节点的右边界坐标
      rect.top     // 节点的上边界坐标
      rect.bottom  // 节点的下边界坐标
      rect.width   // 节点的宽度
      rect.height  // 节点的高度
    }).exec()
  },
  getAllRects () {
    tt.createSelectorQuery().selectAll('.class-name').boundingClientRect(function(rects){
      rects.forEach(function(rect){
        rect.id      // 节点的ID
        rect.dataset // 节点的dataset
        rect.left    // 节点的左边界坐标
        rect.right   // 节点的右边界坐标
        rect.top     // 节点的上边界坐标
        rect.bottom  // 节点的下边界坐标
        rect.width   // 节点的宽度
        rect.height  // 节点的高度
      })
    }).exec()
  },
  getScrollOffset: function(){
    tt.createSelectorQuery().selectViewport().scrollOffset(function(res){
      res.id      // 节点的ID
      res.dataset // 节点的dataset
      res.scrollLeft // 节点的水平滚动位置
      res.scrollTop  // 节点的竖直滚动位置
    }).exec()
  },
  getFields: function(){
    tt.createSelectorQuery().select('#the-id').fields({
      id: true,
      dataset: true,
      size: true,
    }, (res) => {
      res.id         //节点的ID
      res.dataset    // 节点的dataset
      res.width      // 节点的宽度
      res.height     // 节点的高度
    }).exec()
  }
})
```
