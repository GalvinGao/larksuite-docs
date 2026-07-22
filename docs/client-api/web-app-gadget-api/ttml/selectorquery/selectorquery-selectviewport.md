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

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createSelectorQuery/createSelectorQuery" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |


## 输入
无

## 输出

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| NodesRef | object | [NodesRef](/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/nodesref) 对象 |



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



