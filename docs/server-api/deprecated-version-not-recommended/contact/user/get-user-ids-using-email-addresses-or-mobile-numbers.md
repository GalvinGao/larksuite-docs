---
document_id: '6965400907878170629'
directory_id: '6907567266541404162'
title: 使用手机号或邮箱获取用户 ID
full_path: /ukTMukTMukTM/uUzMyUjL1MjM14SNzITN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- User
- Get User IDs Using Email Addresses or Mobile Numbers
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:35Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUzMyUjL1MjM14SNzITN
---

# 使用手机号或邮箱获取用户 ID

根据用户邮箱或手机号查询用户 open_id 和 user_id，支持批量查询。<br>
:::html
<md-alert type="warn">
调用该接口需要申请 `通过手机号或邮箱获取用户 ID` 权限。<br>只能查询到应用可用性范围内的用户 ID，不在范围内的用户会表现为不存在。
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
      <md-td>https://open.larksuite.com/open-apis/user/v1/batch_get_id?emails=lisi@z.com&emails=wangwu@z.com&mobiles=13812345678&mobiles=%2b12126668888</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
    </md-tr>
    
    
    <md-tr>
      <md-th>
 权限要求
 <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
</md-th>
      <md-td>
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">通过手机号或邮箱获取用户 ID </md-perm>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            字段权限要求
            <md-tooltip type="info">接口返回的部分字段受权限控制，开启字段权限才可获取对应字段数据；如无需获取这些字段，则无需开启。</md-tooltip>
            <div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">根据要获取的字段开启相应权限</div>
      </md-th>
      <md-td>
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户 user ID</md-perm>
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户邮箱信息</md-perm>
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户手机号</md-perm>
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
      <md-td> 是 </md-td> 
      	<md-td>
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>
 
**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"
          
 [了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)
	</md-td>
</md-tr>
     <md-tr> 
      <md-td>Content-Type</md-td>  
      <md-td>string</md-td>  
      <md-td> 是 </md-td> 
     <md-td>**固定值**："application/json; charset=utf-8"</md-td>
</md-tr>
   
  </md-tbody> 
</md-table>
:::

### 查询参数
参数 | 类型 | 必填 / 选填 | 示例 | 说明
-- | -- | -- | -- | --
emails | string | 选填 | lisi@z.com | 要查询的用户邮箱，最多 50 条，多个邮箱以 & 隔开。
mobiles | string | 选填 | 13812345678<br>%2b12126668888 | 要查询的用户手机号，最多 50 条，多个手机号以 & 隔开。<br>非中国大陆地区的手机号需要添加以 “+” 开头的国家 / 地区代码，并且需要进行 URL 转义。<br>
## 响应
### 响应体
参数 | 说明
-- | --
code | 返回码，非 0 表示失败。
msg | 对返回码的文本描述。
data | -
&emsp;∟email_users | 根据邮箱查询到的用户，key 为邮箱，value 为查询到用户的 array。<br>目前同一个邮箱最多只能查询到一个用户。
&emsp;&emsp;∟open_id | 用户的 open_id。[open_id描述](/document/home/user-identity-introduction/open-id)
&emsp;&emsp;∟user_id | 用户的 user_id。<br>只有已申请 `获取用户UserID` 权限的企业自建应用返回此字段。[user_id描述](/document/home/user-identity-introduction/user-id)
&emsp;∟emails_not_exist | 没有匹配记录的邮箱。
&emsp;∟mobile_users | 根据手机号查询到的用户，key 为手机号，value 为查询到用户的 array。<br>目前同一个手机号最多只能查询到一个用户。
&emsp;∟mobiles_not_exist | 没有匹配记录的手机号。
### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "email_users": {
            "lisi@z.com": [
                {
                    "open_id": "ou_979112345678741d29069abcdef089d4",
                    "user_id": "a7eb3abe"
                }
            ]
        },
        "emails_not_exist": [
            "wangwu@z.com"
        ],
        "mobile_users": {
            "13812345678": [
                {
                    "open_id": "ou_46a087654321a1dc920ffab8fedc823f",
                    "user_id": "18fg19d4"
                }
            ]
        },
        "mobiles_not_exist": [
            "13912345678",
            "+12126668888"
        ]
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
