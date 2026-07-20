---
document_id: '6965400907878252549'
directory_id: '6916079000750178306'
title: 获取企业自定义用户属性配置
full_path: /ukTMukTMukTM/ucTN3QjL3UzN04yN1cDN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- Obtain Custom User Properties
document_type: GuideDocumentType
updated_at: 2022-03-11T11:45:14Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ucTN3QjL3UzN04yN1cDN
---

# 获取企业自定义用户属性配置
:::html

<md-alert type="error">

为了更好地提升该接口的安全性，我们对其进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/custom_attr/list)

</md-alert>

:::

该接口用于获取企业配置的自定义用户属性。<br>


::: note
 1. 调用该接口前，需要先确认企业管理员已经在 [**企业管理后台** > **组织架构**>**成员字段管理**](http://www.feishu.cn/admin/contacts/employee-field-new/custom)  自定义字段管理栏开启了“允许开放平台通讯录API调用“。
 2. 调用该接口的应用需要具有当前企业通讯录的读取或者更新权限。
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
      <md-td>https://open.larksuite.com/open-apis/contact/v1/tenant/custom_attr/get</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
    </md-tr>
    
    
    <md-tr>
      <md-th>
权限要求
 <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
<div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div>
</md-th>
      <md-td>
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 获取部门组织架构信息 </md-perm>
        <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 读取通讯录 </md-perm>
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
## 响应
### 响应体

|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|返回码的描述|
|data|-|返回业务数据|
|&emsp;∟is_open|bool|企业是否自定义用户属性|
|&emsp;∟custom_attrs|list|当 is_open 的值为 true 时返回此字段。<br>此字段为企业配置的自定义用户属性，每个属性包括平台生成的自定义属性 ID 和企业设置的国际化属性名称。<br>zh_cn、en_us、ja_jp 分别表示中文、英文、日文属性名称|
### 响应体示例
```json
{
    "code": 0,
    "msg": "success",
    "data": {
        "is_open": true,
        "custom_attrs": {
            "C-6702376000044400907": {
                "zh_cn": "自定义属性1",
                "en_us":"custom_attr1",
                "ja_jp":"カスタム属性1"
            },
            "C-6702376000048595214": {
                "zh_cn": "自定义属性2",
                "en_us": "custom_attr2",
                "ja_jp":"カスタム属性2"
            }
        }
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
