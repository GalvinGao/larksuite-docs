---
document_id: '6967331158355689478'
directory_id: '7122028361538912262'
title: 三方审批实例同步
full_path: /ukTMukTMukTM/uczM3UjL3MzN14yNzcTN
breadcrumb:
- Server API
- Approval
- Approval（history version）
- v2
- Third-party approval integration
- External Approval Instance Create
document_type: GuideDocumentType
updated_at: 2022-07-20T09:39:25Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uczM3UjL3MzN14yNzcTN
---

# 三方审批实例同步
:::html
<md-alert type="error">
为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/create)
</md-alert>
:::
<br>
审批中心不负责审批的流转，审批的流转在三方系统，三方系统在审批流转后生成的审批实例、审批任务、审批抄送数据同步到审批中心。<br>
<br>
用户可以在审批中心中浏览三方系统同步过来的实例、任务、抄送信息，并且可以跳转回三方系统进行更详细的查看和操作，其中实例信息在【已发起】列表，任务信息在【待审批】和【已审批】列表，抄送信息在【抄送我】列表
:::html
<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9dff4434afbeb0ef69de7f36b9a6e995_z5iwmTzEgg.png" alt="" style="zoom:17%;" />


<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ca6e0e984a7a6d64e1b16a0bac4bf868_tfqjCiaJQM.png" alt="" style="zoom:17%;" />


<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/529377e238df78d391bbd22e962ad195_T7eefLI1GA.png" alt="" style="zoom:17%;" />
:::
<br>
对于审批任务，三方系统也可以配置审批任务的回调接口，这样审批人可以在审批中心中直接进行审批操作，审批中心会回调三方系统，三方系统收到回调后更新任务信息，并将新的任务信息同步回审批中心，形成闭环。
:::html
<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/721c35428bc1187db3318c572f9979ad_je75QpElcg.png" alt=""  style="zoom:25%;" />


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
      <md-td>https://www.larksuite.com/approval/openapi/v2/external/instance/create</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>

   <md-tr>
     <md-th>支持的应用类型</md-th>
      <md-td>
	  <md-app-support types="custom,isv"></md-app-support>
      </md-td>
   </md-tr>


    
    
    <md-tr>
      <md-th>
 权限要求
 <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
</md-th>
      <md-td>
<md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm>
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

:::html
<md-table>
  <md-thead>
      <tr>
      <md-th>名称</md-th>
      <md-th>类型</md-th>
      <md-th>描述</md-th>
      
      </tr>
  </md-thead>
  <md-tbody>


<md-tr>
	<md-td>
	<md-text type="field-name" >content</md-text>
<md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	JSON字符串
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::

**三方审批JSON**
:::html
<md-table>
  <md-thead>
      <tr>
      <md-th style="width: 22%;">名称</md-th>
      <md-th style="width: 15%;">类型</md-th>
      <md-th>描述</md-th>
      <md-th style="width: 23%;">示例</md-th>
      </tr>
  </md-thead>
  <md-tbody>


<md-tr>
	<md-td>
	<md-text type="field-name" >approval_code</md-text>
<md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>审批定义 code， 创建审批定义返回的值，表示该实例属于哪个流程；该字段会影响到列表中该实例的标题，标题取自对应定义的 name 字段
	</md-td>
  <md-td>
	81D31358-93AF-92D6-7425-01A5D67C4E71
	</md-td>
</md-tr>
    

<md-tr>
	<md-td>
	<md-text type="field-name" >instance_id</md-text>
<md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	审批实例唯一标识，用户自定义，需确保证租户、应用下唯一

	</md-td>
  <md-td>
	24492654
	</md-td>
</md-tr>

<md-tr>
	<md-td>
	<md-text type="field-name" >status</md-text>
<md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
      
	</md-td>
	<md-td>
审批实例状态  
**可选值：**  
- `PENDING`: 审批中
- `APPROVED`: 审批流程结束，结果为同意
- `REJECTED`: 审批流程结束，结果为拒绝
- `CANCELED`: 审批发起人撤回
- `DELETED`: 审批被删除
 - `HIDDEN`: 状态隐藏(不显示状态)<br>

	</md-td>
  <md-td>
PENDING
	</md-td>
</md-tr>

<md-tr>
	<md-td>
	<md-text type="field-name" >extra</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	审批实例扩展 JSON
	</md-td>
  <md-td>
	{"xxx":"xxx"}
	</md-td>
</md-tr>

    
<md-tr>
	<md-td>
	<md-text type="field-name" >links</md-text>
      <md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >map</md-text>
	</md-td>
	<md-td>
	审批实例链接集合 ，用于【已发起】列表的跳转，跳转回三方系统；<br>
pc_link 和 mobile_link 必须填一个，填写的是哪一端的链接，即会跳转到该链接，不受平台影响
	</md-td>
  <md-td>
	
	</md-td>
</md-tr>
 <md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_link
      </md-text><md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
pc 端的跳转链接，当用户使用Lark pc 端时，使用该字段进行跳转
	</md-td>
   <md-td>
https://applink.larksuite.com/client/mini_program/open?mode=appCenter&appId=cli_9c9c9c38e07a9101&path=pc/pages/detail?id=1234
	</md-td>
</md-tr>   
  <md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile_link</md-text><md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
移动端 跳转链接，当用户使用Lark 移动端时，使用该字段进行跳转
	</md-td>
    <md-td>
https://applink.larksuite.com/client/mini_program/open?appId=cli_9c9cfc38e07a9101&path=pages/detail?id=1234
	</md-td>
</md-tr>  
<md-tr>
	<md-td>
	<md-text type="field-name" >title</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
审批展示名称，如果填写了该字段，则审批列表中的审批名称使用该字段，如果不填该字段，则审批名称使用审批定义的名称
	</md-td>
  
  <md-td>
@i18n@1
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	<md-text type="field-name" >form</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >list</md-text>
	</md-td>
	<md-td>
	用户提交审批时填写的表单数据，用于所有审批列表中展示。可传多个值，但审批中心pc展示前2个,移动端展示前3个,长度不超过2048字符
	</md-td>
  <md-td>
	[
    {<br>
      "name": "@i18n@2",<br>
      "value": "@i18n@3"<br>
    }
]
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
表单字段名称
	</md-td>
  <md-td>
@i18n@2
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >value</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
表单值
	</md-td>
  <md-td>
@i18n@3
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	<md-text type="field-name" >user_id</md-text><md-tag mode="block" type="field-required" ></md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
审批发起人 user_id，发起人可在【已发起】列表中看到所有已发起的审批;<br>在【待审批】，【已审批】【抄送我】列表中，该字段展示审批是谁发起的。审批发起人 open id，和 user id 二选一。
	</md-td>
  <md-td>
a987sf9s
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	<md-text type="field-name" >user_name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
审批发起人 用户名，如果发起人不是真实的用户（例如是某个部门），没有 user_id，则可以使用该字段传名称
	</md-td>
  <md-td>
@i18n@9
	</md-td>
</md-tr>
<md-tr>
	<md-td>
	<md-text type="field-name" >open_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
审批发起人 open id，和 user id 二选一
	</md-td>
  <md-td>
ou_be73cbc0ee35eb6ca54e9e7cc14998c1
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	<md-text type="field-name" >department_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
发起人部门，用于列表中展示发起人所属部门。
不传则不展示。
如果用户没加入任何部门，传 ""，将展示租户名称
传 department_name 展示部门名称 
	</md-td>
  <md-td>
od-8ec33278bc2
	</md-td>
</md-tr>
    <md-tr>
	<md-td>
	<md-text type="field-name" >department_name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
审批发起人 部门，如果发起人不是真实的用户（例如是某个部门），没有 department_id，则可以使用该字段传名称
	</md-td>
      <md-td>
@i18n@10
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	<md-text type="field-name" >start_time</md-text>
<md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >long</md-text>
	</md-td>
	<md-td>
审批发起时间，Unix毫秒时间戳
	</md-td>
   <md-td>
1556468012678
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	<md-text type="field-name" >end_time</md-text>
      <md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >long</md-text>
	</md-td>
	<md-td>
审批实例结束时间：未结束的审批为 0，Unix毫秒时间戳
	</md-td>
  <md-td>
1556468012678
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	<md-text type="field-name" >update_time</md-text>
      <md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >long</md-text>
	</md-td>
	<md-td>
审批实例最近更新时间；用于推送数据版本控制
如果 update_mode 值为 UPDATE，则只有传过来的 update_time 有变化时（变大），才会更新审批中心中的审批实例信息。
使用该字段主要用来避免并发时老的数据更新了新的数据
	</md-td>
  <md-td>
1556468012678
	</md-td>
</md-tr>
 <md-tr>
	<md-td>
	<md-text type="field-name" >display_method</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
列表页打开审批实例的方式。
**可选值：**
- `BROWSER`: 跳转系统默认浏览器打开
- `SIDEBAR`: Lark中侧边抽屉打开
- `NORMAL`: Lark内嵌页面打开
<br>
	</md-td>
  <md-td>
BROWSER
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	<md-text type="field-name" >update_mode</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
更新方式  
**可选值：**
- `REPLACE`: 全量替换，默认值
- `UPDATE`: 增量更新
 <br>当 update_mode=REPLACE时，每次都以当前推送的数据为最终数据，会删掉审批中心中多余的任务、抄送数据（不在这次推送的数据中）;<br>
当 update_mode=UPDATE时，则不会删除审批中心的数据，而只是进行新增和更新实例、任务数据
	</md-td>
  <md-td>
UPDATE
	</md-td>
</md-tr>

<md-tr>
	<md-td>
	<md-text type="field-name" >task_list</md-text>
      <md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >list</md-text>
	</md-td>
	<md-td>
任务列表
	</md-td>
  <md-td>
   备注: 任务数限制 最大不超过200
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >task_id</md-text>
      <md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
审批实例内的唯一标识，用于更新审批任务时定位数据
	</md-td>
<md-td>
112534
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text>
      <md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	审批人 user_id，该任务会出现在审批人的【待审批】或【已审批】列表中
	</md-td>
  <md-td>
a987sf9s
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
审批人 open id，和 user id 二选一
	</md-td>
  <md-td>
ou_be73cbc0ee35eb6ca54e9e7cc14998c1
	</md-td>
</md-tr>
   <md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
审批任务名称
	</md-td>
     <md-td>
i18n1
	</md-td>
</md-tr> 
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >links</md-text><md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >map</md-text>
	</md-td>
	<md-td>
【待审批】或【已审批】中使用的跳转链接，用于跳转回三方系统
pc_link 和 mobile_link 必须填一个，填写的是哪一端的链接，即会跳转到该链接，不受平台影响

	</md-td>
  <md-td>

	</md-td>
</md-tr>
    <md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_link</md-text><md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
pc 端的跳转链接，当用户使用Lark pc 端时，使用该字段进行跳转

	</md-td>
  <md-td>
https://applink.larksuite.com/client/mini_program/open?mode=appCenter&appId=cli_9c9cfc38e07a9101&path=pc/pages/detail?id=1234
	</md-td>
</md-tr>
    <md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile_link</md-text><md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
移动端 跳转链接，当用户使用Lark 移动端时，使用该字段进行跳转

	</md-td>
  <md-td>
https://applink.larksuite.com/client/mini_program/open?appId=cli_9c9cfc38e07a9101&path=pages/detail?id=1234
	</md-td>
</md-tr>
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >status</md-text>
<md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
任务状态  
**可选值：**
- `PENDING`: 待审批
- `APPROVED`: 任务同意
- `REJECTED`: 任务拒绝
- `TRANSFERRED`: 任务转交
- `DONE`: 任务通过但审批人未操作；审批人看不到这个任务, 若想要看到, 可以通过抄送该人.
	</md-td>
  <md-td>

	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >extra</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
扩展 json
	</md-td>
  <md-td>

	</md-td>
</md-tr>

<md-tr>
	<md-td>
&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >create_time</md-text>
      <md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >long</md-text>
	</md-td>
	<md-td>
任务创建时间，Unix 毫秒时间戳
	</md-td>
  <md-td>
1556468012678
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >end_time</md-text>
      <md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >long</md-text>
	</md-td>
	<md-td>
任务完成时间：未结束的审批为 0，Unix 毫秒时间戳
	</md-td>
  <md-td>
1556468012678
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >update_time</md-text> <md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >long</md-text>
	</md-td>
	<md-td>
task最近更新时间，用于推送数据版本控制；<br>
更新策略同 instance 中的 update_time
	</md-td>
  <md-td>
1556468012678
	</md-td>
</md-tr>
    <md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >action_context</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
操作上下文，当用户操作时，回调请求中带上该参数，用于传递该任务的上下文数据
	</md-td>
      <md-td>
123456
	</md-td>
</md-tr>
    
    <md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >action_configs</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >list</md-text>
	</md-td>
	<md-td>
任务级别操作配置,快捷审批目前支持移动端操作
	</md-td>
      <md-td>

	</md-td>
</md-tr>
    
        <md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" > action_type </md-text>
      <md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
操作类型，每个任务都可以配置2个操作，会展示审批列表中，当用户操作时，回调请求会带上该字段，表示用户进行了同意操作还是拒绝操作<br>
 **APPROVE** - 同意<br>
 **REJECT** - 拒绝<br>
**{KEY}** - 任意字符串，如果使用任意字符串，则需要提供 action_name

	</md-td>
          <md-td>
APPROVE
	</md-td>
</md-tr>
    
       
            <md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" > action_name </md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
操作名称，i18n key 用于前台展示，如果 action_type 不是 APPROVAL和REJECT，则必须提供该字段，用于展示特定的操作名称
	</md-td>
   <md-td>
@i18n@5
	</md-td>
</md-tr>
            <md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" > is_need_reason  </md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >bool</md-text>
	</md-td>
	<md-td>
是否需要意见, 如果为true,则用户操作时，会跳转到 意见填写页面
	</md-td>
              <md-td>
false
	</md-td>
</md-tr>
            <md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" > is_reason_required  </md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >bool</md-text>
	</md-td>
	<md-td>
审批意见是否必填
	</md-td>
               <md-td>
false
	</md-td>
</md-tr>
                <md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" > is_need_attachment   </md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >bool</md-text>
	</md-td>
	<md-td>
意见是否支持上传附件
	</md-td>
                   <md-td>
false
	</md-td>
</md-tr>
 <md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >display_method</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
列表页打开审批任务的方式。
**可选值：**
- `BROWSER`: 跳转系统默认浏览器打开
- `SIDEBAR`: Lark中侧边抽屉打开
- `NORMAL`: Lark内嵌页面打开
<br>
	</md-td>
  <md-td>
BROWSER
	</md-td>
</md-tr>   
<md-tr>
	<md-td>
	<md-text type="field-name" >cc_list</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >list</md-text>
	</md-td>
	<md-td>
		抄送列表
	</md-td>
   <md-td>
   备注: 抄送数限制 最大不超过200
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >cc_id</md-text>
      <md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
		审批实例内唯一标识
	</md-td>
  <md-td>
		12345
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text>
      <md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
		抄送人  employee id

	</md-td>
  <md-td>
		12345
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
抄送人 open id，和user id 二选一
	</md-td>
 
<md-td>
ou_be73cbc0ee35eb6ca54e9e7cc14998c1
	</md-td>
</md-tr>

<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >links</md-text>
      <md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >map</md-text>
	</md-td>
	<md-td>
跳转链接，用于【抄送我的】列表中的跳转
pc_link 和 mobile_link 必须填一个，填写的是哪一端的链接，即会跳转到该链接，不受平台影响
	</md-td>
  <md-td>


	</md-td>
</md-tr>
    <md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;&nbsp;<md-text type="field-name" >pc_link</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
pc 端的跳转链接，当用户使用Lark pc 端时，使用该字段进行跳转

	</md-td>
  <md-td>
https://applink.larksuite.com/client/mini_program/open?mode=appCenter&appId=cli_9c9cfc38e07a9101&path=pc/pages/detail?id=1234
	</md-td>
</md-tr>
    <md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;&nbsp;<md-text type="field-name" >mobile_link</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
移动端 跳转链接，当用户使用Lark 移动端时，使用该字段进行跳转

	</md-td>
  <md-td>
https://applink.larksuite.com/client/mini_program/open?appId=cli_9c9cfc38e07a9101&path=pages/detail?id=1234
	</md-td>
</md-tr>
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >read_status</md-text>
      <md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	阅读状态，空值表示不支持已读未读：  
      **可选值：**
- `READ`:  已读
- `UNREAD`: 未读 
  </md-td>
  <md-td>
	READ
  </md-td>
</md-tr>
    
   <md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >extra</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
		扩展 json
	</md-td>
     <md-td>
		{<br>"xxx":"xxx"<br>}
	</md-td>
</md-tr>
    
       <md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >title</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
		抄送任务名称
	</md-td>
         <md-td>
		xxx
	</md-td>
</md-tr>
    

<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >create_time</md-text><md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >long</md-text>
	</md-td>
	<md-td>
抄送发起时间，Unix 毫秒时间戳
	</md-td>
  <md-td>
1556468012678
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >update_time</md-text><md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >long</md-text>
	</md-td>
	<md-td>
抄送最近更新时间，用于推送数据版本控制
更新策略同 instance 的update_time
	</md-td>
  <md-td>
1556468012678
	</md-td>
</md-tr>
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >display_method</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
列表页打开审批任务的方式。
**可选值：**
- `BROWSER`: 跳转系统默认浏览器打开
- `SIDEBAR`: Lark中侧边抽屉打开
- `NORMAL`: Lark内嵌页面打开
<br>
	</md-td>
  <md-td>
BROWSER
	</md-td>
</md-tr>      
<md-tr>
	<md-td>
<md-text type="field-name" >i18n_resources</md-text><md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >list</md-text>
	</md-td>
	<md-td>
国际化文案
	</md-td>
   <md-td>

	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >locale</md-text><md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
语言
- `zh-CN`: 中文
- `en-US`: 英文
- `ja-JP`: 日文
	</md-td>
  <md-td>
zh-CN
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_default</md-text><md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >bool</md-text>
	</md-td>
	<md-td>
		是否默认语言
	</md-td>
   <md-td>
true
	</md-td>
</md-tr>
    
<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >texts</md-text><md-tag mode="block" type="field-required" >必选</md-tag>
	</md-td>
	<md-td>
	<md-text type="field-type" >map</md-text>
	</md-td>
	<md-td>
		文案 key, value,  i18n key 以  @i18n@ 开头；<br>
该字段主要用于做国际化，语序用户同时传多个语言的文案，审批中心会根据用户当前的语音环境使用对应的文案，如果没有传用户当前的语音环境文案，则会使用默认的语言文案。

	</md-td>
  <md-td>
	{<br>
   "@i18n@1": "权限申请",<br>      "@i18n@2": "OA审批",<br>
   "@i18n@3": "Permission"<br>
}
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::

注意事项：
- 需确保审批实例内各类唯一 ID 在审批实例内的唯一性，即使不属于同一实体。

```json
{
    "content": "{\"approval_code\":\"60D0F7A2-052D-49E9-B570-C29C836CDF8E\",\"instance_id\":\"216263\",\"status\":\"PENDING\",\"extra\":\"\",\"links\":{\"pc_link\":\"http://182.50.118.46:38080/spa/workflow/static4form/index.html?#/main/workflow/req?requestid=\",\"mobile_link\":\"http://182.50.118.46:38080/spa/workflow/static4form/index.html?#/main/workflow/req?requestid=\"},\"title\":\"@i18n@1\",\"form\":[{\"name\":\"@i18n@2\",\"value\":\"@i18n@3\"}],\"user_id\":\"52d6585f\",\"user_name\":\"342a\",\"open_id\":\"123\",\"department_id\":\"od-8ec33ffec336c3a39a278bc25e931676\",\"department_name\":\"XXXX\",\"start_time\":1622444502000,\"update_time\":1622444502000,\"end_time\":0,\"update_mode\":\"REPLACED\",\"task_list\":[{\"task_id\":\"112253\",\"user_id\":\"52d6585f\",\"links\":{\"pc_link\":\"http://\",\"mobile_link\":\"http://\"},\"status\":\"PENDING\",\"extra\":\"\",\"title\":\"同意\",\"create_time\":1566468012678,\"end_time\":0,\"update_time\":1566468012678,\"action_context\":\"123456\",\"action_configs\":[{\"action_type\":\"APPROVE\",\"action_name\":\"@i18n@9\",\"is_need_reason\":true,\"is_reason_required\":true,\"is_need_attachment\":true}]}],\"cc_list\":[{\"cc_id\":\"1231243\",\"user_id\":\"a987sf9s\",\"open_id\":\"\",\"links\":{\"pc_link\":\"http://\",\"mobile_link\":\"http://\"},\"read_status\":\"READ\",\"extra\":\"\",\"title\":\"XXX\",\"create_time\":1556468012678,\"update_time\":1566468012678}],\"i18n_resources\":[{\"locale\":\"zh-CN\",\"is_default\":true,\"texts\":{\"@i18n@1\":\"离开\",\"@i18n@2\":\"天\",\"@i18n@3\":\"2020-08-01\"}},{\"locale\":\"en-US\",\"is_default\":false,\"texts\":{\"@i18n@1\":\"Leave\",\"@i18n@2\":\"Day\",\"@i18n@3\":\"2020-08-01\"}},{\"locale\":\"ja-JP\",\"is_default\":false,\"texts\":{\"@i18n@1\":\"Leave\",\"@i18n@2\":\"Day\",\"@i18n@3\":\"2020-08-01\"}}]}"
}
```




## 响应

### 响应体

|参数|类型|必须|说明|
|-|-|-|-|
|code|int|是|返回码，非0表示失败|
|msg|String|是|返回码的描述|
|data|map|是|返回业务信息|

### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {
    }
}
```
