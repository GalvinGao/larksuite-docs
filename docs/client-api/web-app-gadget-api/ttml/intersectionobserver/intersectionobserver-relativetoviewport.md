---
document_id: '7073692582769164294'
directory_id: '7073451436033835013'
title: IntersectionObserver.relativeToViewport
full_path: /uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/relativetoviewport
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- IntersectionObserver
- IntersectionObserver.relativeToViewport
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:54Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/relativetoviewport
---

# IntersectionObserver.relativeToViewport(object margins)

指定页面显示区域作为参照区域之一


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V2.3.0+</md-version> | <md-version>V2.3.0+</md-version> | <md-version>V2.3.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="这里一定要修改成具体的路径" fontSize="14">预览</md-preview-app> |
| 网页应用 | **✕** | **✕** | **✕** | / |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| margins | object | 是 |  | 用来扩展（或收缩）参照节点布局区域的边界 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>left<br></md-text> | number | 否 | 0 | 节点布局区域的左边界 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>right<br></md-text> | number | 否 | 0 | 节点布局区域的右边界 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>top<br></md-text> | number | 否 | 0 | 节点布局区域的上边界 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>bottom<br></md-text> | number | 否 | 0 | 节点布局区域的下边界 |


## 输出
无


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createIntersectionObserver/createIntersectionObserver" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.createIntersectionObserver(this, {
    selectAll: true
})
.relativeToViewport({top:10})
```

