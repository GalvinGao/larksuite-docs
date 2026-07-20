---
document_id: '6965400907875270662'
directory_id: '6907567266541404162'
title: 新增用户
full_path: /ukTMukTMukTM/uMzNz4yM3MjLzczM
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- User
- Add a User
document_type: GuideDocumentType
updated_at: 2022-03-11T11:44:20Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uMzNz4yM3MjLzczM
---

# 新增用户

:::html

<md-alert type="error">

为了更好地提升该接口的安全性，我们对其进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/create)

</md-alert>

:::
该接口用于向通讯录中新增用户。  



- 调用该接口需要申请`更新通讯录`以及`以应用身份访问通讯录`权限。新增用户的所有部门必须都在当前应用的通讯录授权范围内才允许新增用户。应用商店应用无权限调用此接口



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
      <md-td>https://open.larksuite.com/open-apis/contact/v1/user/add</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>
    
    
    <md-tr>
      <md-th>
权限要求
 <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
<div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div>
</md-th>
      <md-td>
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 更新通讯录 </md-perm>
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份访问通讯录（历史版本）</md-perm>
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
### 请求体 
|参数|类型|必须|说明|
|-|-|-|-|
|name|string|是|用户名|
|email|string|否|用户邮箱地址，同一企业内，邮箱必须唯一|
|mobile|string|是|用户手机号，同一企业内，手机号必须唯一|
|department_ids、open_department_ids|list|否|新增用户所在部门自定义 ID 或者 openID，二者至少提供其中之一，**同时传两个参数优先使用 open_department_ids**，当前版本接口仅支持一个用户创建在一个部门下，需要有加入部门的通讯录权限；|
|mobile_visible|bool|否|手机号码可见性，true 为可见，false 为不可见，目前默认为 true。不可见时，组织员工将无法查看该员工的手机号码|
|city|string|否|用户所在城市|
|country|string|否|用户所在国家|
|gender|string|否|性别，1:男，2:女|
|employee_type|int|否|员工类型。1:正式员工；2:实习生；3:外包；4:劳务；5:顾问|
|join_time|int|否|入职时间|
|leader_employee_id、leader_open_id|string|否|直接领导信息，支持通过 leader_employee_id 或者 leader_open_id 设置直接领导，**同时传递两个参数时按参数 leader_employee_id 处理**|
|employee_id|string|否|用户企业内唯一标识。<br>自定义唯一标识不区分大小写，长度为 1 ~ 64 个字符。只能由数字、字母和 "_-@.“ 四种字符组成，且第一个字符必须是数字或字母。<br>创建用户时可指定该唯一标识，指定的唯一标识不能修改。|
|employee_no|string|否|工号|
|need_send_notification|bool|否|是否发送邀请通知。该字段为 true 时， 添加用户成功后会往相应的邮箱或者 mobile 发送邀请通知|
|custom_attrs|object|否|自定义用户属性。<br>该字段仅当企业管理员在企业管理后台开启了“允许开放平台API调用”时有效。 <br>传入的每个自定义用户属性需要包含平台生成的属性ID和要设置的属性值。<br>当企业管理后台未开启“允许开放平台API调用”，以及传入的自定义用户属性 ID 不存在或者非法时，会忽略该条属性设置信息。|
|work_station|string|否|工位|
### 请求体示例 
```json
{
    "name":"张三",
    "department_ids":["TT-1234"],
    "open_department_ids": ["od-a528a1df6182b09110887ef653bd5e03"],
    "email":"zhangsan@gmail.com",
    "mobile":"+8613822235671",
    "mobile_visible":false,
    "city":"北京",
    "country": "CN",
    "gender":1,
    "employee_type":1,
    "join_time":1534222800,
    "leader_open_id":"ou_3454556545acdb12324",
    "leader_employee_id":"2ab56f23",
    "employee_id":"tt_123456",
    "employee_no":"323463",
    "need_send_notification":true,
    "custom_attrs":{
        "C-6702376000044400907": {
            "value": "value1"
        },
        "C-6702376000048595214": {
            "value": "value2"
        }
    },
    "work_station":"Poly, F6-123"
}
```




## 响应
### 响应体
|参数|说明|
|-|-|
|code|返回码，非 0 表示失败|
|msg|返回码的描述|
|data|返回业务信息|
|&emsp;∟userinfo|用户信息|
|&emsp;&emsp;∟employee_id|用户的 employee_id，企业自建应用返回，应用商店应用不返回|
|&emsp;&emsp;∟open_id|用户的 open_id|
|&emsp;&emsp;∟name|用户名|
|&emsp;&emsp;∟status|用户状态|
|&emsp;&emsp;∟avatar_72|用户头像，72*72px|
|&emsp;&emsp;∟avatar_240|用户头像，240*240px|
|&emsp;&emsp;∟avatar_640|用户头像，640*640px|
|&emsp;&emsp;∟avatar_url|用户头像，原始大小|
|&emsp;&emsp;∟update_time|更新时间|
|&emsp;&emsp;∟email|用户邮箱地址|
|&emsp;&emsp;∟gender|string|否|性别，1:男，2:女|
|&emsp;&emsp;∟join_time|int|否|入职时间戳|
|&emsp;&emsp;∟en_name|英文名|
|&emsp;&emsp;∟description|用户描述信息|
|&emsp;&emsp;∟departments|用户所在部门的自定义 ID，用户可能同时存在于多个部门|
|&emsp;&emsp;∟open_departments|用户所在部门的 openID，用户可能同时存在于多个部门|
|&emsp;&emsp;∟city|用户所在城市|
|&emsp;&emsp;∟country|用户所在国家|
|&emsp;&emsp;∟station|工位信息|
|&emsp;&emsp;∟leader_employee_id|直接领导 employee_id，企业自建应用返回，应用商店应用不返回|
|&emsp;&emsp;∟leader_open_id|直接领导 open_id|
|&emsp;&emsp;∟work_station|工位|

### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "user_info":{
            "employee_id":"tt_123456",
            "open_id":"ou_6dfd8d7e5e881bed9be19c043940bf60",
            "name":"张三",
            "status":2,
            "avatar_72": "https://sf3-ttcdn-tos.pstatp.com/img/avatar/62db96e8-c5b6-4077-bb9d-2697d65a29eb~72x72.png",
            "avatar_240": "https://sf3-ttcdn-tos.pstatp.com/img/avatar/62db96e8-c5b6-4077-bb9d-2697d65a29eb~240x240.png",
            "avatar_640": "https://sf3-ttcdn-tos.pstatp.com/img/avatar/62db96e8-c5b6-4077-bb9d-2697d65a29eb~640x640.png",
            "avatar_url": "https://sf3-ttcdn-tos.pstatp.com/img/avatar/62db96e8-c5b6-4077-bb9d-2697d65a29eb~noop.png",
            "update_time":1540886759,
            "name_py":"zhangsan",
            "email":"zhangsan@gmail.com",
            "gender":1,
            "join_time":"1534222800",
            "departments":[
                "TT-1234"
            ],
            "open_departments":[
                "od-a140b4eeb892b90a0ab3e616fc2054d6"
            ],
            "city":"北京",
            "country": "CN",
            "work_station":"Poly, F6-123",
            "leader_employee_id":"2ab56f23",
            "leader_open_id":"ou_76a95e7f1b7841f74576d2fc00838f47"
        }
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
