---
document_id: '6965379541104181253'
directory_id: '7345445193132867590'
title: login
full_path: /uYjL24iN/uYzMuYzMuYzM
breadcrumb:
- Client API
- Deprecated Version (Not Recommended)
- login
document_type: GuideDocumentType
updated_at: 2022-03-11T04:12:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYzMuYzMuYzM
---

# login(Object object)


获取临时登录凭证。完整的登录流程参考[小程序登录](/document/uYjL24iN/uETO5QjLxkTO04SM5kDN)

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/login/login" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性

## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| code | string | 临时登录凭证，有效期 3 分钟，只能使用一次 |


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/login/login" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.login({
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`login fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "login:ok",
    "code": "4473273da6ae7d15"
}
```

## 错误码
`fail` 返回对象中会包含 `errCode` 属性，代表错误码，具体错误码列表参见：
错误码  | 描述
--|--|--|--|--
1101301  | 接口重复调用导致。建议等待前序调用结果返回后进行下次调用
