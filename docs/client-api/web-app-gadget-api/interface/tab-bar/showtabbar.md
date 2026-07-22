---
document_id: '6965379541104623621'
directory_id: '6907567269107367938'
title: showTabBar
full_path: /uYjL24iN/uATN04CM1QjLwUDN
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Tab Bar
- showTabBar
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/uATN04CM1QjLwUDN
---

# showTabBar(Object object)


显示 tabBar（小程序底部tab栏）


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/index?showTabBarPage=true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| animation | boolean | 否 | false | 是否需要动画效果。<br><md-alert type="tip" icon="none"><br>PC 端：暂不支持<br></md-alert> |


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
tt.showTabBar({
    animation: true,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`showTabBar fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "showTabBar:ok"
}
```
