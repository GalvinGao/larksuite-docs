---
document_id: '6965379543684071430'
directory_id: '6907567266541240322'
title: showModal
full_path: /uYjL24iN/ugDNy4CO0IjL4QjM
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Interaction Feedback
- showModal
document_type: GuideDocumentType
updated_at: 2024-01-12T06:34:29Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDNy4CO0IjL4QjM
---

# showModal(Object object)

showModal(Object object) 用于显示模态弹窗。

## 支持说明

该接口支持小程序和网页应用调用，对应的客户端版本支持情况如下所示。

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/modal/modal" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.47.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |



## 输入

该接口继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性如下所示。

| 名称 | 数据类型 | 是否必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| title | string | 否 | \- | 弹窗的标题。最多可显示两行。<br><md-alert type="tip"><br>**注意**：<br>- `title`和`content`不可同时为空。<br>- 显示效果在各端有差异，请你根据实际内容自行调试各端的显示效果。<br></md-alert> |
| content | string | 否 | \- | 弹窗的内容。<br><md-alert type="tip"><br>**注意**：<br>- `title`和`content`不可同时为空。<br>- 显示效果在各端有差异，请你根据实际内容自行调试各端的显示效果。<br></md-alert> |
| confirmText | string | 否 | OK | **确定** 按钮的文案，中文按照 2 个字符计算。<br><md-alert type="tip"><br>Lark [V7.8.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)版本以前，最大长度为`8`个字符<br></md-alert> |
| cancelText | string | 否 | Cancel | **取消** 按钮的文案，中文按照 2 个字符计算。<br><md-alert type="tip"><br>Lark [V7.8.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)版本以前，最大长度为`8`个字符<br></md-alert> |
| showCancel | boolean | 否 | true | 是否显示 **取消** 按钮。取值：<br>- true：显示<br>- false：不显示 |



## 输出

该接口继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success` 返回对象的扩展属性如下所示。

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| confirm | boolean | 是否点击了 **确定** 按钮。 |
| cancel | boolean | 是否点击了 **取消** 按钮。 |


## 示例代码

调用示例：

:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/modal/modal" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.showModal({
    "title": "请求获得定位权限",
    "content": "获得你的地理位置能够更好的为你推荐本地信息",
    "confirmText": "授予权限",
    "cancelText": "取消",
    "showCancel": true,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`showModal fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{
    "errMsg": "showModal:ok",
    "confirm": true,
    "cancel": false
}
``` 

## 错误码

`fail` 返回对象中可能包含 errno 属性，表示错误码。关于 errno 错误码的详细说明以及通用错误码列表，可参见[Errno 错误码](/document/uYjL24iN/uAjMuAjMuAjM/errno)。
