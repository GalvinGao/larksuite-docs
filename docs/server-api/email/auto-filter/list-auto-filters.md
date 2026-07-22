---
document_id: '7496871390063525900'
directory_id: '7494547571672678406'
title: 列出收信规则
full_path: /uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-rule/list
breadcrumb:
- Server API
- Email
- Auto Filter
- List Auto FIlters
document_type: ReferenceDocumentType
updated_at: 2025-04-28T11:38:56Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-rule/list
---

# 列出收信规则

列出收信规则{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=mail&version=v1&resource=user_mailbox.rule&method=list)

:::html
<md-alert type="tip">
使用 tenant_access_token 时，需要申请收信规则资源的数据权限。
</md-alert>
:::

:::html
<md-alert type="warn">

</md-alert>
:::

:::html
<md-alert type="error">

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules |
| HTTP Method | GET |
| 接口频率限制 | [5 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="mail:user_mailbox.rule:read" desc="查看收信规则" support_app_types="custom" tags="">查看收信规则</md-perm><br><md-perm name="mail:user_mailbox.rule:write" desc="查看、创建、更新、删除收信规则" support_app_types="custom" tags="">查看、创建、更新、删除收信规则</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |




### 路径参数
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
	<md-text type="field-name" >user_mailbox_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户邮箱地址 或 输入me代表当前调用接口用户

**示例值**："user@xxx.xx 或 me"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
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
	<md-text type="field-name" >msg</md-text>
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
	<md-text type="field-type" >\-</md-text>
	</md-dt-td>
	<md-dt-td>
	\-
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >items</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >rule\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	规则列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	规则 id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >condition</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >rule_condition</md-text>
	</md-dt-td>
	<md-dt-td>
	匹配条件
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >match_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	匹配类型

**可选值有**：
<md-enum>
<md-enum-item key="1" >满足所有条件</md-enum-item>
<md-enum-item key="2" >满足任意条件</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >items</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >rule_condition_item\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	匹配规则列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	匹配条件左值

**可选值有**：
<md-enum>
<md-enum-item key="1" >发件人地址</md-enum-item>
<md-enum-item key="2" >收件人地址</md-enum-item>
<md-enum-item key="3" >抄送地址</md-enum-item>
<md-enum-item key="4" >收件人或抄送地址</md-enum-item>
<md-enum-item key="6" >主题</md-enum-item>
<md-enum-item key="7" >正文</md-enum-item>
<md-enum-item key="8" >附件名字</md-enum-item>
<md-enum-item key="9" >附件类型</md-enum-item>
<md-enum-item key="10" >任意地址</md-enum-item>
<md-enum-item key="12" >所有邮件</md-enum-item>
<md-enum-item key="13" >是外部邮件</md-enum-item>
<md-enum-item key="14" >是垃圾邮件</md-enum-item>
<md-enum-item key="15" >不是垃圾邮件</md-enum-item>
<md-enum-item key="16" >有附件</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >operator</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	匹配条件操作符

**可选值有**：
<md-enum>
<md-enum-item key="1" >包含</md-enum-item>
<md-enum-item key="2" >不包含</md-enum-item>
<md-enum-item key="3" >开头是</md-enum-item>
<md-enum-item key="4" >结尾是</md-enum-item>
<md-enum-item key="5" >是</md-enum-item>
<md-enum-item key="6" >不是</md-enum-item>
<md-enum-item key="7" >包含自己</md-enum-item>
<md-enum-item key="10" >为空</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >input</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	匹配条件右值
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >action</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >rule_action</md-text>
	</md-dt-td>
	<md-dt-td>
	匹配命中后的操作
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >items</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >rule_action_item\[\]</md-text>
	</md-dt-td>
	<md-dt-td>
	匹配中规则后的操作列表
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	操作类型

**可选值有**：
<md-enum>
<md-enum-item key="1" >归档</md-enum-item>
<md-enum-item key="2" >删除邮件</md-enum-item>
<md-enum-item key="3" >标记为已读</md-enum-item>
<md-enum-item key="4" >移至垃圾邮件</md-enum-item>
<md-enum-item key="5" >不移至垃圾邮件</md-enum-item>
<md-enum-item key="8" >添加用户标签（暂不支持）</md-enum-item>
<md-enum-item key="9" >添加旗标</md-enum-item>
<md-enum-item key="10" >不弹出通知</md-enum-item>
<md-enum-item key="11" >移至用户文件夹</md-enum-item>
<md-enum-item key="12" >自动转发（暂不支持）</md-enum-item>
<md-enum-item key="13" >分享到会话（暂不支持）</md-enum-item>
</md-enum>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="4">
	<md-dt-td>
	<md-text type="field-name" >input</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	当 type 为移动到文件夹时，该字段填文件夹的 id
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >ignore_the_rest_of_rules</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否终点规则
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	规则名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >is_enable</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	是否启用
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
    "msg": "success",
    "data": {
        "items": [
            {
                "id": "123124123123",
                "condition": {
                    "match_type": 1,
                    "items": [
                        {
                            "type": 1,
                            "operator": 1,
                            "input": "hello@world.com"
                        }
                    ]
                },
                "action": {
                    "items": [
                        {
                            "type": 1,
                            "input": "283412371233"
                        }
                    ]
                },
                "ignore_the_rest_of_rules": false,
                "name": "将李三的邮件标记为垃圾邮件",
                "is_enable": false
            }
        ]
    }
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 400 | 1230001 | 参数错误 | 修改参数后重试 |
| 500 | 1230003 | 内部错误 | 请稍后重试 |
| 403 | 1230002 | 无权限 | 成为公共邮箱成员或申请相关数据权限后调用该接口 |





