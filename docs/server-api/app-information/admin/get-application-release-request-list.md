---
document_id: '7070695615567200262'
directory_id: '6907567266537291777'
title: 查看待审核的应用列表
full_path: /uAjLw4CM/ukTMukTMukTM/application-v6/application/underauditlist
breadcrumb:
- Server API
- App Information
- Admin
- Get Application Release Request List
document_type: ReferenceDocumentType
updated_at: 2022-03-03T02:30:09Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/underauditlist
---

# 查看待审核的应用列表

查看本企业下所有待审核的自建应用列表{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=application&version=v6&resource=application&method=underauditlist)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">

</md-alert>
:::

:::html
<md-alert type="tip">

</md-alert>
:::



## 请求
:::html
<md-table>
  <md-thead>
  <tr>
      <md-th>基本</md-th>
      <md-th></md-th>
  </tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-th>HTTP URL</md-th>
      <md-td>https://open.larksuite.com/open-apis/application/v6/applications/underauditlist</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
    </md-tr>

   <md-tr>
     <md-th>支持的应用类型</md-th>
      <md-td>
	  <md-app-support types="custom"></md-app-support>
      </md-td>
   </md-tr>


    <md-tr>
      <md-th>
 权限要求
 <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
</md-th>
      <md-td>
<md-perm name="admin:app.info:readonly" desc="获取应用信息" support_app_types="custom" tags="">获取应用信息</md-perm>
</md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::


### 请求头
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 18%;">名称</md-th>
      <md-th style="width: 15%;">类型</md-th>
       <md-th style="width: 15%;">必填</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>Authorization</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      	<md-td>
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"

[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)
</md-td>
</md-tbody>
</md-table>
:::



### 查询参数
:::html
<md-table>
  <md-thead>
      <tr>
      <md-th style="width: 30%;">名称</md-th>
      <md-th style="width: 15%;">类型</md-th>
      <md-th style="width: 15%;">必填</md-th>
      <md-th >描述</md-th>
      </tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >lang</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	指定返回的语言

**示例值**："zh_cn"

**可选值有**：
- `zh_cn`：中文
- `en_us`：英文
- `ja_jp`：日文

**数据校验规则**：

- 最小长度：`1` 字符
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >page_token</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果

**示例值**："new-e3c5a0627cdf0c2e057da7257b90376a"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >page_size</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	分页大小

**示例值**：10

**数据校验规则**：
- 最大值：`50`
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::





## 响应



### 响应体
:::html
<md-table>
  <md-thead>
      <md-tr>
      <md-th style="width: 40%;">名称</md-th>
      <md-th style="width: 20%;">类型</md-th>
      <md-th style="width: 30%;">描述</md-th>
      </md-tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >code</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	错误码，非 0 表示失败
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >msg</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	错误描述
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >data</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >\-</md-text>
	</md-td>
	<md-td>
	\-
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >items</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >application\[\]</md-text>
	</md-td>
	<md-td>
	待审核应用列表
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	应用的 app_id
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >creator_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	应用创建者（所有者）
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >status</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	应用状态

**可选值有**：
- `0`：停用状态
- `1`：启用状态
- `2`：未启用状态
- `3`：未知状态
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >scene_type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	应用类型

**可选值有**：
- `0`：自建应用
- `1`：应用商店应用
- `2`：个人应用商店应用
- `3`：未知应用类型
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >redirect_urls</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-td>
	<md-td>
	安全设置中的重定向 URL
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >online_version_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	发布在线上的应用版本 ID，若没有则为空
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >unaudit_version_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	在审核中的版本 ID，若没有则为空
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	应用名称

**数据校验规则**：

- 最小长度：`1` 字符
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >avatar_url</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	应用图标 url
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	应用默认描述

**数据校验规则**：

- 最小长度：`1` 字符
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >scopes</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >app_scope\[\]</md-text>
	</md-td>
	<md-td>
	应用权限列表
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >scope</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	应用权限
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	应用权限的国际化描述
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >level</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	权限等级描述

**可选值有**：
- `1`：普通权限
- `2`：高级权限
- `3`：超敏感权限
- `0`：未知等级
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >back_home_url</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	后台主页地址
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >app_i18n_info\[\]</md-text>
	</md-td>
	<md-td>
	应用的国际化信息列表

**数据校验规则**：

- 最小长度：`1`
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n_key</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	国际化语言的 key

**可选值有**：
- `zh_cn`：中文
- `en_us`：英文
- `ja_jp`：日文
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	应用国际化名称

**数据校验规则**：

- 最小长度：`1` 字符
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	应用国际化描述（副标题）

**数据校验规则**：

- 最小长度：`1` 字符
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >help_use</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	帮助国际化文档链接
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >primary_language</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	应用主语言

**可选值有**：
- `zh_cn`：中文
- `en_us`：英文
- `ja_jp`：日文
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >common_categories</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-td>
	<md-td>
	应用分类的国际化描述

**数据校验规则**：

- 最大长度：`3`
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >has_more</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >boolean</md-text>
	</md-td>
	<md-td>
	是否还有更多项
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >page_token</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::



### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "items": [
            {
                "app_id": "cli_9b445f5258795107",
                "creator_id": "ou_d317f090b7258ad0372aa53963cda70d",
                "status": 1,
                "scene_type": 0,
                "redirect_urls": [
                    "https://www.example.com"
                ],
                "online_version_id": "oav_d317f090b7258ad0372aa53963cda70d",
                "unaudit_version_id": "oav_d317f090b7258ad0372aa53963cda70d",
                "app_name": "应用名称",
                "avatar_url": "https://sf1-ttcdn-tos.pstatp.com/img/avatar/d279000ca4d3f7f6aaff~72x72.jpg",
                "description": "应用描述",
                "scopes": [
                    {
                        "scope": "contact:user.base",
                        "description": "获取应用信息",
                        "level": "low_level"
                    }
                ],
                "back_home_url": "https://www.example.com",
                "i18n": [
                    {
                        "i18n_key": "zh_cn",
                        "name": "应用名称",
                        "description": "应用描述",
                        "help_use": "https://www.example.com"
                    }
                ],
                "primary_language": "zh_cn",
                "common_categories": [
                    "分析工具"
                ]
            }
        ],
        "has_more": true,
        "page_token": "new-xxxxxxxxxxx"
    }
}
```



### 错误码
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">HTTP状态码</md-th>
            <md-th style="width: 15%;">错误码</md-th>
            <md-th style="width: 30%;">描述</md-th>
            <md-th style="width: 30%;">排查建议</md-th>
        </md-tr>
    </md-thead>
  <md-tbody>

<md-tr>
  <md-td>400</md-td>
  <md-td>210500</md-td>
  <md-td>page_token does not exist or has expired</md-td>
  <md-td>请检查 page_token 是否合法，page_token 过期时间为 2h，若超过 2h 请重新获取</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>210501</md-td>
  <md-td>invalid page_token</md-td>
  <md-td>page_token 在应用间不互通，请检查该 page_token 是否由调用接口的应用获取到</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>210502</md-td>
  <md-td>page_size out of range, should be between 1 and 50</md-td>
  <md-td>请检查 page_size 范围是否在 [1, 50] 范围内</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




