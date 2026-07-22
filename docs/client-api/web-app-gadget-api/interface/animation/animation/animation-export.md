---
document_id: '7073691561007759365'
directory_id: '7073450228347305989'
title: Animation.export
full_path: /uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_export
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Animation
- Animation
- Animation.export
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_export
---

# Animation.export()

导出动画队列。`export` 方法每次调用后会清掉之前的动画操作

:::html
<md-alert>每次 `export` 只会导出「尚未被导出」的动画组，若某动画组已经被导出过，则会被清除。如果在调用 `export` 时存在尚未完成的「动画组」，则未进入「动画组」的视觉变换不会生效（但也不会被删除，下次调用 `step` 方法后会继续生效）。</md-alert>
:::


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/animation/animation" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入
无


## 输出

返回值：  

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| actions | string | 动画队列 |


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/animation/animation" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
const animation = tt.createAnimation();

animation.background('#FFFFFF').bottom(20).step();
animation.export();
```
