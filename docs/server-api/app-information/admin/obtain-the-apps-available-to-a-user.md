---
document_id: '6971799133842341893'
directory_id: '6907567266537291777'
title: 获取用户可用的应用
full_path: /ukTMukTMukTM/uMjM3UjLzIzN14yMycTN
breadcrumb:
- Server API
- App Information
- Admin
- Obtain the Apps Available to a User
document_type: GuideDocumentType
updated_at: 2022-03-28T03:24:47Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uMjM3UjLzIzN14yMycTN
---

# 获取用户可用的应用

该接口用于查询用户可用的应用列表，只能被企业自建应用调用。

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
      <md-td>https://open.larksuite.com/open-apis/application/v1/user/visible_apps</md-td>
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
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">获取用户可用的应用</md-perm>
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

### 请求参数
|参数|类型|必须|说明|
|--|-----|--|----|
|page_token|string|否|分页起始位置标示，不填表示从头开始|
|page_size|int|否|单页需求最大个数（最大 100），0 自动最大个数|
|lang|string|否|优先展示的应用信息的语言版本（zh_cn：中文，en_us：英文，ja_jp：日文）|
|open_id|string|否|目标用户 open_id|
|user_id|string|否|目标用户 user_id，与 open_id 至少给其中之一，user_id 优先于 open_id|

## 响应

### 响应体
|参数|说明|
|--|--|
|code|返回码，非 0 表示失败|
|msg|返回码的描述|
|data|返回的业务信息，仅 code = 0 时有效|
|&emsp;∟page_token|下一个请求页应当给的起始位置|
|&emsp;∟page_size|本次请求实际返回的页大小|
|&emsp;∟total_count|可用的应用总数|
|&emsp;∟has_more|是否还有更多应用|
|&emsp;∟lang|当前选择的版本语言|
|&emsp;∟app_list|应用列表|
|&emsp;&emsp;∟app_id|应用 ID|
|&emsp;&emsp;∟primary_language|应用首选语言|
|&emsp;&emsp;∟app_name|应用名称|
|&emsp;&emsp;∟description|应用描述|
|&emsp;&emsp;∟avatar_url|应用 icon|
|&emsp;&emsp;∟app_scene_type|应用类型，0：企业自建应用；1：应用商店应用|
|&emsp;&emsp;∟status|启停状态，0：停用；1：启用|
|&emsp;&emsp;∟mobile_default_ability|移动端默认的应用功能，0：未开启；1：小程序；2：H5；8：机器人|
|&emsp;&emsp;∟pc_default_ability|PC客户端默认的应用功能，0：未开启；1：小程序；2：H5；8：机器人|

### 响应示例
```json
{
    "code": 0,
    "data": {
        "app_list": [
            {
                "app_id": "cli_9cb844403dbb9108",
                "app_name": "审批",
                "app_scene_type": 1,
                "avatar_url": "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/05dfa8bd265e3d2e74643484a04a3002_emU7aAr1tD.png?lazyload=true&width=1600&height=778",
                "description": "简单、高效、开放的审批工具",
                "mobile_default_ability": 1,
                "pc_default_ability": 1,
                "primary_language": "zh_cn",
                "status": 1
            },
            {
                "app_id": "cli_9cb844403dbb9108",
                "app_name": "工资单",
                "app_scene_type": 1,
                "avatar_url": "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/05dfa8bd265e3d2e74643484a04a3002_emU7aAr1tD.png?lazyload=true&width=1600&height=778",
                "description": "便捷、安全的工资单管理，一键完成工资发布",
                "mobile_default_ability": 1,
                "pc_default_ability": 1,
                "primary_language": "zh_cn",
                "status": 1
            },
            {
                "app_id": "cli_9cb844403dbb9108",
                "app_name": "问卷网",
                "app_scene_type": 1,
                "avatar_url": "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/05dfa8bd265e3d2e74643484a04a3002_emU7aAr1tD.png?lazyload=true&width=1600&height=778",
                "description": "专业易用的「问卷调研 · 报名表单 · 考试测评」平台",
                "mobile_default_ability": 1,
                "pc_default_ability": 1,
                "primary_language": "zh_cn",
                "status": 1
            }
        ],
        "has_more": 1,
        "lang": "zh_cn",
        "page_size": 3,
        "page_token": "3",
        "total_count": 34
    },
    "msg": "success"
}
```

