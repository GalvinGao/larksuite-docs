---
document_id: '6965379543683858438'
directory_id: '6907567269107367938'
title: hideTabBarRedDot
full_path: /uYjL24iN/ugjM04COyQjL4IDN
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Tab Bar
- hideTabBarRedDot
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:50Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugjM04COyQjL4IDN
---

# 	hideTabBarRedDot(Object object)


隐藏 tabBar（小程序底部tab栏） 某一项的右上角的红点。相关 API：[showTabBarRedDot](/document/uYjL24iN/uYjM04iNyQjL2IDN)


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/index?showTabBarPage=true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| index | number | 是 |  | tabBar 的哪一项，从左边算起<br>**最小值**：`0` |


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
tt.hideTabBarRedDot({
    index: 0,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`hideTabBarRedDot fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "hideTabBarRedDot:ok"
}
```
