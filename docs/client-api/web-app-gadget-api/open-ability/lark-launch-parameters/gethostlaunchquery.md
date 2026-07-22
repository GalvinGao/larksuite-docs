---
document_id: '6967331158356344838'
directory_id: '6907567269107859458'
title: getHostLaunchQuery
full_path: /uYjL24iN/ugzM4UjL4MDO14COzgTN
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Lark Launch Parameters
- getHostLaunchQuery
document_type: GuideDocumentType
updated_at: 2022-03-11T04:13:38Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugzM4UjL4MDO14COzgTN
---

# getHostLaunchQuery(Object object)

获取小程序自定义启动参数。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.10.0+</md-version> | <md-version>V3.10.0+</md-version> | <md-version>V3.10.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-launch-query/get-launch-query" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| launchQuery | string | 自定义传入的参数 |


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-launch-query/get-launch-query" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.getHostLaunchQuery({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getHostLaunchQuery fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "launchQuery": "自定义参数",
    "errMsg": "getHostLaunchQuery:ok"
}
```
