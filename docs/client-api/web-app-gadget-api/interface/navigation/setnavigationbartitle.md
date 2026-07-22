---
document_id: '6965379541104934917'
directory_id: '6907567266536243201'
title: setNavigationBarTitle
full_path: /uYjL24iN/uATNy4CM1IjLwUjM
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Navigation
- setNavigationBarTitle
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:56Z
source_url: https://open.larksuite.com/document/uYjL24iN/uATNy4CM1IjLwUjM
---

# setNavigationBarTitle(Object object)
设置导航栏标题。

:::html 
<md-alert type="tip">
如果当前页面不存在导航栏，不会返回`fail`
</md-alert>
:::


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/set-navigation-bar-title/set-navigation-bar-title" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| title | string | 是 |  | 导航栏标题 |


## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/set-navigation-bar-title/set-navigation-bar-title" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.setNavigationBarTitle({
    "title": "newTitle",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`setNavigationBarTitle fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{
    "errMsg": "setNavigationBarTitle:ok"
}
``` 
