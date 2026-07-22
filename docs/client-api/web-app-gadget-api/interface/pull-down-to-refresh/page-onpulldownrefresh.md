---
document_id: '6965379541104214021'
directory_id: '6907567269107384322'
title: Page.onPullDownRefresh
full_path: /uYjL24iN/uQTNy4CN1IjL0UjM
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Pull Down to Refresh
- Page.onPullDownRefresh
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:02Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQTNy4CN1IjL0UjM
---

# Page.onPullDownRefresh(Object object)

在 Page 中注册下拉刷新的监听方法，当用户触发下拉刷新时会调用。


::: warnning
只有在页面的`page.json`中配置了`enablePullDownRefresh: true`才能够开启下拉刷新。
:::


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/pull-down-refresh/pull-down-refresh" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性

## 输出

无

##  示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/pull-down-refresh/pull-down-refresh" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div>
:::

```js
Page({
    onPullDownRefresh() {
        console.log('onPullDownRefresh')
    }
});
```
