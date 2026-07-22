---
document_id: '6965379543684530182'
directory_id: '6907567266537750529'
title: pageScrollTo
full_path: /uYjL24iN/uITNy4iM1IjLyUjM
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Page Position
- pageScrollTo
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:11Z
source_url: https://open.larksuite.com/document/uYjL24iN/uITNy4iM1IjLyUjM
---

# pageScrollTo(Object object)

滚动页面到目标位置


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/page-scroll-to/page-scroll-to" fontSize="14">预览</md-preview-app> |
| 网页应用 | **✓** | **✓** | **✓** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14" disable="true">预览</md-preview-app> |



## 输入

属性描述：

名称 | 数据类型 | 属性 | 默认值 | 描述
--|--|--|--|--
`scrollTop` | `number` | required | N/A | 位置，单位 `px`
`duration` | `number` | optional | `200` | 执行时长，单位 `ms`

## 输出
无

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/page-scroll-to/page-scroll-to" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
    <md-preview-app type="webApp" fontSize="16" disable="true">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.pageScrollTo({
    scrollTop: 3008,
    duration: 1000,
    success () {
        console.log(`PageScrollTo invoked successfully`);
    },
    fail () {
        console.log(`Failed to invoke pageScrollTo`);
    }
});
```
