---
document_id: '7073692582769426438'
directory_id: '6907567269107367938'
title: removeTabBarItem
full_path: /uYjL24iN/uQjM04CNyQjL0IDN/removetabbaritem
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Tab Bar
- removeTabBarItem
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:35Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQjM04CNyQjL0IDN/removetabbaritem
---

# 	removeTabBarItem(Object object)


删除tab bar（小程序底部tab栏）的目标item


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V4.2.0+</md-version> | <md-version>V4.2.0+</md-version> | <md-version>V4.2.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/index?showTabBarPage=true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| tag | string | 否 |  | 删除tab bar的目标item<br>**示例值**：'pages/index/index' |


## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/index?showTabBarPage=true" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.removeTabBarItem({
    tag: "pages/index/index",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`removeTabBarItem fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "removeTabBarItem:ok"
}
```
