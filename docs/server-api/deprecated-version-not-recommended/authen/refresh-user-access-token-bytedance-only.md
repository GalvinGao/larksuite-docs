---
document_id: '7520163088822878214'
directory_id: '7345445193132883974'
title: 刷新 user_access_token（仅字节）
full_path: /uAjLw4CM/ukTMukTMukTM/authentication-management/access-token/refresh-user_access_token-bytedance
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- authen
- Refresh user_access_token (Bytedance only)
document_type: GuideDocumentType
updated_at: 2025-06-26T07:53:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/authentication-management/access-token/refresh-user_access_token-bytedance
---

# 刷新 user_access_token（仅字节）

:::html
<md-alert type="error">
本接口已成为历史版本，不推荐使用。请使用最新版本：[刷新 user_access_token](/document/uAjLw4CM/ukTMukTMukTM/authentication-management/access-token/refresh-user-access-token)
</md-alert>。
:::

user_access_token 的最大有效期是 2小时左右。当 user_access_token 过期时，可以调用本接口获取新的 user_access_token。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=authen&version=v1&resource=oidc.refresh_access_token&method=create)

:::html
<md-alert type="tip">

</md-alert>
:::

:::html
<md-alert type="warn">
- 2024年9月23日起，字节内新创建的企业自建应用将**默认无法通过「获取 user_access_token」接口获取 refresh_token**，需在应用开发者后台安全设置中开启「刷新 user_access_token」开关后方可正常获取和刷新。9月19日前创建的企业自建应用暂不受影响，可通过原接口直接获取和刷新。

- 刷新后请更新本地user_access_token和refresh_token，不要继续使用旧值重复刷新。保证参数是最新值
</md-alert>
:::

:::html
<md-alert type="error">

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/authen/v1/oidc/refresh_access_token |
| HTTP Method | POST |
| 接口频率限制 | [1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | 无 |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-app">app_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer a-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 请求体

:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >grant_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	授权类型，**固定值**：

**示例值**："refresh_token"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >refresh_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	刷新和获取user_access_token接口均返回 `refresh_token`，**每次请求，请注意使用最新获取到的`refresh_token`**

**示例值**："ur-h4_5nUXdJ4O8rqfGe.YJCwM13Gjc557xUG20hkk00f7K"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "grant_type": "refresh_token",
    "refresh_token": "ur-h4_5nUXdJ4O8rqfGe.YJCwM13Gjc557xUG20hkk00f7K"
}
</md-code-json>
:::



## 响应





### 响应体
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 52%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	错误码，非 0 表示失败
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >message</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	错误描述
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >data</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >token_info</md-text>
	</md-dt-td>
	<md-dt-td>
	\-
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >access_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	字段`access_token`即user_access_token，用于获取用户资源和访问某些open api
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >refresh_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	刷新 user_access_token时使用的 refresh_token
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >token_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	token 类型，固定值
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >expires_in</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	user_access_token有效期，单位: 秒，有效时间两个小时左右，需要以返回结果为准
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >refresh_expires_in</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	refresh_token有效期，单位: 秒，一般是30天左右，需要以返回结果为准
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >scope</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户授予app的权限全集
	</md-dt-td>
</md-dt-tr>


  </md-dt-tbody>
</md-dt-table>
:::



### 响应体示例
:::html
<md-code-json>
{
    "code": 0,
    "message": "success",
    "data": {
        "access_token": "u-5Dak9ZAxJ9tFUn8MaTD_BFM51FNdg5xzO0y010000HWb",
        "refresh_token": "ur-6EyFQZyplb9URrOx5NtT_HM53zrJg59HXwy040400G.e",
        "token_type": "Bearer",
        "expires_in": 7199,
        "refresh_expires_in": 2591999,
        "scope": "auth:user.id:read bitable:app"
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 200 | 20001 | Invalid request. Please check request param | 请检查请求参数 |
| 200 | 20002 | The app_id or app_secret passed is incorrect. Please check the value | 检查app_id和密钥是否正确 |
| 200 | 20007 | Failed to generate a user access token. Please try again | 请检查参数是否有效，重试 |
| 200 | 20008 | User not exist | 用户不存在，换有效帐号 |
| 200 | 20013 | The tenant access token passed is invalid. Please check the value | 检查tenant_access_token是否有效 |
| 200 | 20014 | The app access token passed is invalid. Please check the value | 检查app_access_token是否有效 |
| 200 | 20021 | User resigned | 用户离职，请使用有效帐号 |
| 200 | 20022 | User frozen | 用户冻结，请使用有效帐号 |
| 200 | 20023 | User not registered | 用户未注册，请使用有效帐号 |
| 200 | 20024 | App id in user_access_token or refresh_token diff with app id in app_access_token or tenant_access_token. Please keep the app id consistent | 请检查生成两个token的app是否为同一个 |
| 200 | 20026 | The refresh token passed is invalid. Please check the value | 无效refresh_token，请检查是否过期或已经消费 |
| 200 | 20028 | Invalid app id | 无效app_id，请检查参数 |
| 200 | 20029 | Invalid redirect uri | redirect_uri 无效。排查方案：<br>1. 确保 Authorization 取值正确。<br>2. 确保[获取登录授权码 code](/document/common-capabilities/sso/api/obtain-oauth-code) 时，设置的回调地址 redirect_uri 参数，已配置到开发者后台 > 应用详情页 > 安全设置 > 重定向 URL。<br>关于该报错的详细解决方案，参见[如何解决授权免登页面 20029 错误](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-resolve-the-authorization-page-20029-error)。 |
| 200 | 20036 | The grant_type passed is not supported | 无效grant_type，请与接口要求保持一致 |
| 200 | 20037 | The refresh token passed has expired. Please generate a new one | 过期refresh_token，请传有效参数 |
| 200 | 20038 | The refresh token passed is not found. Please check the value | 查询不到refresh_token，请传有效参数 |
| 200 | 20042 | App disabled | app不可用，请检查app状态 |
| 200 | 20046 | Brand inconsistency | 应用品牌和域名品牌不一致，请保证lark应用在lark域名下使用 |





