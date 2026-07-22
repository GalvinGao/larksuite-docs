---
document_id: '6965379541104967685'
directory_id: '6907567266541699074'
title: getUserInfo
full_path: /uYjL24iN/ucjMx4yNyEjL3ITM
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- UserInfo
- getUserInfo
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:08Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucjMx4yNyEjL3ITM
---

# getUserInfo(Object object)

getUserInfo(Object object) 用于获取已登录用户的基本信息或特殊信息。

## 注意事项

- 小程序调用该接口前，需要确保已经调用 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 LarkV6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) )。
- 网页应用需要在[鉴权](/document/uYjL24iN/uEzM4YjLxMDO24SMzgjN)后调用该接口。
- 如果你的Lark版本为 [V5.2.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上，在调用该接口前需要用户授权 `scope.userInfo`。
	
    - 了解如何授权，可参见 [API 权限](/document/uYjL24iN/uITMuITMuITM)。
    - 作为应用的开发者，你需要兼容用户拒绝授权的场景。



## 支持说明

该接口支持小程序和网页应用调用，对应的客户端版本支持情况如下所示。

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-user-info/get-user-info" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |


## 输入
该接口继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性如下所示。

| 名称 | 数据类型 | 是否必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| withCredentials | boolean | 否 | false | 是否需要返回敏感数据。取值：<br>- true：需要<br>- false：不需要<br><md-alert type="tip" icon="none"><br>网页应用不支持该字段。网页应用获取用户敏感信息的方式，可参见 [网页应用免登](/document/uYjL24iN/ukTO4UjL5kDO14SO5gTN#6efde855)。<br></md-alert> |


## 输出

该接口继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success` 返回对象的扩展属性如下所示。

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| userInfo | object | 用户信息。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>nickName<br></md-text> | string | 用户昵称。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>avatarUrl<br></md-text> | string | 用户头像。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>gender<br></md-text> | string | 用户性别。可能值：<br>- `''`：未知<br>- `male`：男性<br>- `female`：女性 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>country<br></md-text> | string | 用户所在国家或地区。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>city<br></md-text> | string | 用户所在城市。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>language<br></md-text> | string | 所在地区所用的语言。可选值：<br>- `en_US`：英文<br>- `zh_CN`：中文 |
| rawData | string | userInfo 的 JSON 字符串形式。 |
| signature | string | 用于校验用户信息是否被篡改。<br><md-alert type="tip" icon="none"><br>该字段值为敏感数据，输入的 withCredentials 为 true 时可获取。<br></md-alert> |
| encryptedData | string | 包括敏感信息（例如，openId、unionId、email、employee_id、watermark 等）在内的已加密用户数据。<br><md-alert type="tip" icon="none"><br>该字段值为敏感数据，输入的 withCredentials 为 true 时可获取。<br></md-alert> |
| iv | string | 加密算法参数。<br><md-alert type="tip" icon="none"><br>该字段值为敏感数据，输入的 withCredentials 为 true 时可获取。<br></md-alert> |



## 示例代码

调用示例：

:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-user-info/get-user-info" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::
```js
tt.getUserInfo({
    withCredentials: true,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getUserInfo fail: ${JSON.stringify(res)}`);
    }
});
```
`success`返回对象示例：
```json
{
    "userInfo": {
        "avatarUrl": "https://s1-imfile.feishucdn.com/static-resource/v1/c8a42593-e88f-4fdb-8c70-631xxxxxx~?image_size=72x72&cut_type=&quality=&format=png&sticker_format=.webp",
        "city": "武汉",
        "country": "CN",
        "gender": "male",
        "language": "",
        "nickName": "张三"
    },
    "signature": "383292bbece2bc4d0b7591xxxxxxxxxx",
    "encryptedData": "sJbpFAqEo1xjI+gzhQDLqXxxxxxxxxx",
    "rawData": "{\"nickName\":\"张三\",\"avatarUrl\":\"https://s1-imfile.feishucdn.com/static-resource/v1/c8a42593-e88f-4fdb-8c70-631xxxxxx~?image_size=72x72\\u0026cut_type=\\u0026quality=\\u0026format=png\\u0026sticker_format=.webp\",\"gender\":\"male\",\"city\":\"武汉\",\"country\":\"CN\",\"language\":\"\",\"i18nName\":{\"en_us\":\"Zhang San\",\"ja_jp\":\"\",\"zh_cn\":\"\"}}",
    "iv": "bd49a34ccf1e415da113bxxxxxxx",
    "errMsg": "getUserInfo:ok"
}
```

## 错误码

`fail` 返回对象中可能包含 errno 属性，表示错误码。关于 errno 错误码的详细说明以及通用错误码列表，可参见[Errno 错误码](/document/uYjL24iN/uAjMuAjMuAjM/errno)。
