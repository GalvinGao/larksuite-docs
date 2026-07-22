---
document_id: '6965379541104050181'
directory_id: '7073448001569914886'
title: showShareMenu
full_path: /uYjL24iN/ugjN24CO2YjL4YjN
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Share
- showShareMenu
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:57Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugjN24CO2YjL4YjN
---

# showShareMenu(Object object)

显示当前页面的分享按钮

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/share_menu/share_menu" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性

## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/share_menu/share_menu" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::
```js
tt.showShareMenu({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`showShareMenu fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "showShareMenu:ok"
}
```


