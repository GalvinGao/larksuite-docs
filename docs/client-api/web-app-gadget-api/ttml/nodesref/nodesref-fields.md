---
document_id: '7073691561008578565'
directory_id: '7073451436034146309'
title: NodesRef.fields
full_path: /uYjL24iN/uUjN24SN2YjL1YjN/nodesref/fields
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- NodesRef
- NodesRef.fields
document_type: GuideDocumentType
updated_at: 2023-02-10T09:17:21Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/fields
---

# NodesRef.fields(Object object, function callback)

获取节点的相关信息。需要获取的字段在`fields`中指定。返回值是`nodesRef`对应的 `selectorQuery`。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createSelectorQuery/createSelectorQuery" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| fields | object | 是 |  | fields 配置 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>id<br></md-text> | boolean | 否 |  | 是否返回节点 id |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>dataset<br></md-text> | boolean | 否 |  | 是否返回节点 dataset |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>rect<br></md-text> | boolean | 否 |  | 是否返回节点布局位置（left right top bottom） |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>size<br></md-text> | boolean | 否 |  | 是否返回节点尺寸（width height） |
| callback | function | 是 |  | callback function |




## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| id | string | 节点 id |
| dataset | object | 节点的 dataset |
| left | number | 节点的左边界坐标 |
| right | number | 节点的右边界坐标 |
| top | number | 节点的上边界坐标 |
| bottom | number | 节点的下边界坐标 |
| width | number | 节点的宽度 |
| height | number | 节点的高度 |


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
});
```
