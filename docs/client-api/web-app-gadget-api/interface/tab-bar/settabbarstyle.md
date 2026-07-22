---
document_id: '6965379543684251654'
directory_id: '6907567269107367938'
title: setTabBarStyle
full_path: /uYjL24iN/uITN04iM1QjLyUDN
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Tab Bar
- setTabBarStyle
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:38Z
source_url: https://open.larksuite.com/document/uYjL24iN/uITN04iM1QjLyUDN
---

# setTabBarStyle(Object object)


动态设置 tabBar（小程序底部tab栏） 的整体样式


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/index?showTabBarPage=true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| color | string | 是 |  | tab 上的文字默认颜色，6 位 HexColor 例如"#ff3377"<br>**示例值**：'#FF0000' |
| selectedColor | string | 是 |  | tab 上的文字选中时的颜色，6 位 HexColor 例如"#ff3377"<br>**示例值**：'#00FF00' |
| backgroundColor | string | 是 |  | tab 的背景色，6 位 HexColor 例如"#ff3377"<br>**示例值**：'#0000FF' |
| borderStyle | string | 是 | white | tabBar上边框的颜色， 仅支持 black/white<br>**示例值**：'white'<br>**可选值**：<br>- `white`：白<br>- `black`：黑 |
| borderColor | string | 是 |  | tabBar上边框的颜色,优先级高于borderStyle。适用4.3及以后版本。<br>**示例值**：'#00FF00'<br><md-alert type="tip" icon="none"><br>Lark[V4.3.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |


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
tt.setTabBarStyle({
    color: "#FF0000",
    selectedColor: "#00FF00",
    backgroundColor: "#0000FF",
    borderStyle: "white",
    borderColor: "#00FF00",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`setTabBarStyle fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "setTabBarStyle:ok"
}
```
