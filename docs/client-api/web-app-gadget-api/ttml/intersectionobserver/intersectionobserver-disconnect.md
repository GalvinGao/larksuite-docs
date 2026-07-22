---
document_id: '7073692582768951302'
directory_id: '7073451436033835013'
title: IntersectionObserver.disconnect
full_path: /uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/disconnect
breadcrumb:
- Client API
- Web app/Gadget API
- TTML
- IntersectionObserver
- IntersectionObserver.disconnect
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:54Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/disconnect
---

# IntersectionObserver.disconnect()

停止监听，回调函数将不再触发。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V2.3.0+</md-version> | <md-version>V2.3.0+</md-version> | <md-version>V2.3.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/createIntersectionObserver/createIntersectionObserver" fontSize="14">预览</md-preview-app> |
| 网页应用 | **✕** | **✕** | **✕** | / |


## 输入
无

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
const ob = tt.createIntersectionObserver(this, {
    selectAll: true
})
.relativeTo('.container',{top:10})
ob.disconnect()
```

