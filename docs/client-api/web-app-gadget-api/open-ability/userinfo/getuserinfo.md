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

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">应用能力</md-th>
      <md-th style="width: 20%;">Android</md-th>
       <md-th style="width: 20%;">iOS</md-th>
      <md-th style="width: 20%;">PC</md-th>
      <md-th style="width: 20%;">预览效果</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>小程序</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
  <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-user-info/get-user-info" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app></md-td>
</md-tr>  
</md-tbody>
</md-table>
:::

## 输入
该接口继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性如下所示。

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">
                名称
            </md-th>
            <md-th style="width: 15%;">
                数据类型
            </md-th>
            <md-th style="width: 15%;">
                是否必填
            </md-th>
            <md-th style="width: 15%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                withCredentials
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>false</md-td>
            <md-td>
                是否需要返回敏感数据。取值：
- true：需要
- false：不需要
              
<md-alert type="tip" icon="none">
网页应用不支持该字段。网页应用获取用户敏感信息的方式，可参见 [网页应用免登](/document/uYjL24iN/ukTO4UjL5kDO14SO5gTN#6efde855)。
</md-alert>
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出

该接口继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success` 返回对象的扩展属性如下所示。

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 20%;">
                数据类型
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                userInfo
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                用户信息。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    nickName
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户昵称。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    avatarUrl
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户头像。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    gender
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户性别。可能值：
- `''`：未知
- `male`：男性
- `female`：女性
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    country
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户所在国家或地区。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    city
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户所在城市。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    language
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                所在地区所用的语言。可选值：
- `en_US`：英文
- `zh_CN`：中文
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                rawData
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                userInfo 的 JSON 字符串形式。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                signature
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用于校验用户信息是否被篡改。
<md-alert type="tip" icon="none">
该字段值为敏感数据，输入的 withCredentials 为 true 时可获取。
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                encryptedData
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                包括敏感信息（例如，openId、unionId、email、employee_id、watermark 等）在内的已加密用户数据。
<md-alert type="tip" icon="none">
该字段值为敏感数据，输入的 withCredentials 为 true 时可获取。
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                iv
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                加密算法参数。
<md-alert type="tip" icon="none">
该字段值为敏感数据，输入的 withCredentials 为 true 时可获取。
</md-alert>
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


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
