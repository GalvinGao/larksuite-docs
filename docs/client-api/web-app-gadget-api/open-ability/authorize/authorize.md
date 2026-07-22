---
document_id: '6965379541104017413'
directory_id: '6907567266537635841'
title: authorize
full_path: /uYjL24iN/ugzMx4COzEjL4MTM
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Authorize
- authorize
document_type: GuideDocumentType
updated_at: 2022-03-11T04:13:32Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugzMx4COzEjL4MTM
---

# authorize(Object object)

向用户发出设置[权限](/document/uYjL24iN/uITMuITMuITM)请求

:::html
<md-alert type="tip">
注意事项：
- 该权限用户没有设置过，会弹窗咨询用户是否授予；
- 该权限用户设置过，会直接返回结果，不会跟用户产生交互。
</md-alert>
:::


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/authorized/authorized" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| [scope](/document/uYjL24iN/uITMuITMuITM#28328ff6) | string | 是 |  | 需要请求用户设置的权限名称<br>**示例值**：scope.record |

## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/authorized/authorized" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.authorize({
    scope: "scope.record",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`authorize fail: ${JSON.stringify(res)}`);
    }
});
```
`success`返回对象示例：
```json
{
    "errMsg": "authorize:ok"
}
```
