---
document_id: '6965400907878088709'
directory_id: '6920532209305042945'
title: 搜索用户
full_path: /ukTMukTMukTM/uMTM4UjLzEDO14yMxgTN
breadcrumb:
- Server API
- Contacts
- User
- Search Users
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:41Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uMTM4UjLzEDO14yMxgTN
---

# 搜索用户

以用户身份搜索其他用户的信息，无法搜索到外部企业或已离职的用户。<br>
:::html
<md-alert type="warn">
调用该接口需要申请 `搜索用户` 权限。
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
      <md-td>https://open.larksuite.com/open-apis/search/v1/user?query=zhangsan&page_size=20</md-td>
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
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 搜索用户 </md-perm>
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
<md-tag mode="inline" type="token-user">user_access_token</md-tag>
 
**值格式**："Bearer `access_token`"

**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"
          
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
query | string | 必填 | zhangsan | 要执行搜索的字符串，一般为用户名。
page_size | int | 选填 | 20 | 分页大小，最小为 1，最大为 200，默认为 20。
page_token | string | 选填 | 20 | 分页标识，获取首页不需要填写，获取下一页时传入上一页返回的分页标识值。<br>请注意此字段的值并没有特殊含义，请使用每次请求所返回的标识值。
## 响应
### 响应体
参数 | 说明
-- | --
code | 返回码，非 0 表示失败。
msg | 对返回码的文本描述。
data | -
&emsp;∟has_more | 是否还有更多用户，值为 true 表示存在下一页。
&emsp;∟page_token | 分页标识，存在下一页的时候返回。下次请求带上此标识可以获取下一页的用户。
&emsp;∟users | 搜索到的用户列表。
&emsp;&emsp;∟avatar | 用户的头像信息。
&emsp;&emsp;&emsp;∟avatar_72 | 用户的头像图片 URL，72×72px。
&emsp;&emsp;&emsp;∟avatar_240 | 用户的头像图片 URL，240×240px。
&emsp;&emsp;&emsp;∟avatar_640 | 用户的头像图片 URL，640×640px。
&emsp;&emsp;&emsp;∟avatar_origin | 用户的头像图片 URL，原始大小。
&emsp;&emsp;∟department_ids | 用户所在的部门 ID。
&emsp;&emsp;∟name | 用户名。
&emsp;&emsp;∟open_id | 用户的 open_id。
&emsp;&emsp;∟user_id | 用户的 user_id，只有已申请 `获取用户UserID` 权限的企业自建应用返回此字段。
### 响应体示例
```json
{
    "code": 0,
    "msg": "ok",
    "data": {
        "has_more": true,
        "page_token": "20",
        "users": [
            {
                "avatar": {
                    "avatar_72": "https://sf6-ttcdn-tos.pstatp.com/img/lark.avatar/d1ca00148ad2c2cf62ed~72x72.png",
                    "avatar_240": "https://sf6-ttcdn-tos.pstatp.com/img/lark.avatar/d1ca00148ad2c2cf62ed~240x240.png",
                    "avatar_640": "https://sf6-ttcdn-tos.pstatp.com/img/lark.avatar/d1ca00148ad2c2cf62ed~640x640.png",
                    "avatar_origin": "https://lf3-ttcdn-tos.pstatp.com/img/lark.avatar/d1ca00148ad2c2cf62ed~noop.png"
                },
                "department_ids": [
                    "od-c02cc3b682a71cdb3a4f14fc4cdb76ac"
                ],
                "name": "zhangsan",
                "open_id": "ou_7d8a6e6df7621556ce0d21922b676706",
                "user_id": "02a13a9d"
            }
        ]
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
