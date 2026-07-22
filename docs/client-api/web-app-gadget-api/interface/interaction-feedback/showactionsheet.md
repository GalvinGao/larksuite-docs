---
document_id: '6965379541104345093'
directory_id: '6907567266541240322'
title: showActionSheet
full_path: /uYjL24iN/ukDNy4SO0IjL5QjM
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Interaction Feedback
- showActionSheet
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:01Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDNy4SO0IjL5QjM
---

# showActionSheet(Object object)

显示操作菜单。

:::html 
<md-alert type="tip">
iOS 实现时会自动加入「取消」选项，android 3.37（包含）版本之后会自动加入「取消」选项。
</md-alert>
:::


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/action-sheet/action-sheet" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| itemList | string[] | 是 |  | 菜单的选项，最多支持**6**个选项 |


## 输出
`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| tapIndex | number | 用户点击次序，从`0`开始计数 |



## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/action-sheet/action-sheet" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.showActionSheet({
    "itemList": [
        "item1",
        "item2",
        "item3",
        "item4"
    ],
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`showActionSheet fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{
    "errMsg": "showActionSheet:ok",
    "tapIndex": 1
}
``` 

## 已知问题
- 每个选项文案长度限制：
	- Android没有限制，超过长度的文案会显示为...；
	- iOS每个选项最多`1`行，每行约`18`个汉字。
