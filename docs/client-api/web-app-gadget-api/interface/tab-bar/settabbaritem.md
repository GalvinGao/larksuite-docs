---
document_id: '6965379543684382726'
directory_id: '6907567269107367938'
title: setTabBarItem
full_path: /uYjL24iN/uETN04SM1QjLxUDN
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Tab Bar
- setTabBarItem
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:29Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETN04SM1QjLxUDN
---

# 	setTabBarItem(Object object)

动态设置 tabBar（小程序底部tab栏） 某一项的内容


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
| text | string | 否 |  | tab 上的按钮文字<br>**示例值**：'text' |
| iconPath | string | 否 |  | 图片路径，icon 大小限制为 40kb，建议尺寸为 96px * 96px，当 postion 为 top 时，此参数无效，不支持网络图片<br>**示例值**：'/path/to/iconPath' |
| selectedIconPath | string | 否 |  | 选中时的图片路径，icon 大小限制为 40kb，建议尺寸为 96px * 96px ，当 postion 为 top 时，此参数无效<br>**示例值**：'/path/to/selectedIconPath' |


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
tt.setTabBarItem({
    index: 0,
    text: "text",
    iconPath: "./image/icon_API.png",
    selectedIconPath: "./image/icon_API.png",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`setTabBarItem fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "setTabBarItem:ok"
}
```
