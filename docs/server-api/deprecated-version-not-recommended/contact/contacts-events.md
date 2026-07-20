---
document_id: '6965400907878187013'
directory_id: '6916079000750178306'
title: 通讯录事件
full_path: /ukTMukTMukTM/uITNxYjLyUTM24iM1EjN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- Contacts Events
document_type: GuideDocumentType
updated_at: 2021-05-23T08:32:56Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uITNxYjLyUTM24iM1EjN
---

# 通讯录事件
:::html
<md-alert type="tip">
了解事件订阅的使用场景和配置流程，请点击查看 [事件订阅概述](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)
</md-alert>
:::
## 开发前必看
-      如果订阅了通讯录变更事件就会发送旧版通讯录变更事件
-      如果订阅了部门被创建、部门被删除、部门被更新、用户入职、用户离职、用户被更新等事件会收到新版通讯录变更事件


## 用户状态变更
当员工的激活、暂停账号/恢复账号、操作离职时会触发此事件。此事件不依赖于任何权限。

**回调示例：**
```json
{
	"ts": "1502199207.7171419", //  事件发送的时间，一般近似于事件发生的时间。 
	"uuid": "bc447199585340d1f3728d26b1c0297a",  // 事件的唯一标识
	"token": "41a9425ea7df4536a7623e38fa321bae", // 即Verification Token 
	"type": "event_callback", // 此事件此处始终为event_callback
	"event": {
		"type": "user_status_change",    // 事件类型
		"app_id": "cli_xxx",   // 应用ID
		"tenant_key": "xxx",   // 企业标识 
		"open_id":"xxx" ,  // 员工对此应用的唯一标识，同一员工对不同应用的open_id不同
		"employee_id":"xxx",    // 即“用户ID”，仅企业自建应用会返回
		"union_id": "xxx",  // 员工对此ISV的唯一标识，同一员工对同一个ISV名下所有应用的union_id相同
		"before_status": { // 变化前的状态
			"is_active": false,        // 账号是否已激活
			"is_frozen": false,       // 账号是否冻结
			"is_resigned": false    // 是否离职
		},
		"change_time": "2020-02-21 16:28:48", // 状态更新的时间
		"current_status": { // 变化后的状态
			"is_active": true,
			"is_frozen": false,
			"is_resigned": false
		}
	}
}
```

附：不同操作对应的状态变化：
| 操作         | is_active           | is_frozen         | is_resigned |
| --------- | --------------- | -------   | ----------- |
|加入企业后激活账号 | ==false== -> ==true== | - | - |
|暂停账号 | - | ==false== -> ==true== | - |
|恢复账号 | - | ==true== -> ==false== | - |
|操作离职 | - | - | ==false== -> ==true== |

## 通讯录变更
### 员工变更
当员工加入企业（user_add）、离职（user_leave）、个人信息发生变化（user_update）时，推送此事件。
- 依赖权限：==以应用身份访问通讯录==
- 其他条件：只有在企业通讯录授权范围内的员工变化才会推送事件。
- 搭配使用：[获取员工信息接口](/document/ukTMukTMukTM/uIzNz4iM3MjLyczM)

**回调示例：**
```json
{ 
    "ts": "1502199207.7171419", //  事件发送的时间，一般近似于事件发生的时间。 
    "uuid": "bc447199585340d1f3728d26b1c0297a",  // 事件的唯一标识
    "token": "41a9425ea7df4536a7623e38fa321bae", // 即Verification Token 
    "type": "event_callback", // 此事件此处始终为event_callback
    "event": { 
         "type": "user_add",    // 事件类型，包括user_add, user_update, user_leave 
         "app_id": "cli_xxx",   // 应用ID
         "tenant_key": "xxx",   // 企业标识 
         "open_id":"xxx" ,  // 员工对此应用的唯一标识，同一员工对不同应用的open_id不同
         "employee_id":"xxx",    // 即“用户ID”，仅企业自建应用会返回
         "union_id": "xxx" // 员工对此ISV的唯一标识，同一员工对同一个ISV名下所有应用的union_id相同
    }  
}
```

### 部门变更
当新建部门（dept_add）、删除部门（dept_delete）、修改部门（dept_update）时，推送此事件。
- 依赖权限：==以应用身份访问通讯录==
- 其他条件：只有在企业通讯录授权范围内的部门变化才会推送事件。
- 搭配使用：[获取部门详情接口](/document/ukTMukTMukTM/uAzNz4CM3MjLwczM)

**回调示例：**
```json
{ 
    "ts": "1502199207.7171419", //  事件发送的时间，一般近似于事件发生的时间。
    "uuid": "bc447199585340d1f3728d26b1c0297a",  // 事件的唯一标识
    "token": "41a9425ea7df4536a7623e38fa321bae", // 即Verification Token 
    "type": "event_callback", // 此事件此处始终为event_callback
     "event": { 
         "type": "dept_add",  // 事件类型，包括 dept_add, dept_update, dept_delete 
         "app_id": "cli_xxx",  // 应用ID 
         "tenant_key": "xxx",           // 企业标识 
         "open_department_id":"aaa",  // 部门的Id，已废弃
         "department": {
         		"open_id": "od-xxx",
                "custom_id": "aaa"
         }
     } 
}
```



## 授权范围变更

当应用申请了 `以应用身份访问通讯录` 权限后，管理员可以配置应用的通讯录授权范围：

![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/9e6699c282c5964d7e6aba0fca6f14ae.png)
当此范围变化时，就会触发授权范围变化事件。
- 依赖权限：==以应用身份访问通讯录==
- 特殊说明：此事件无需订阅，开通了==以应用身份访问通讯录== 权限后自动订阅。
- 搭配使用：[获取通讯录授权范围](/document/ukTMukTMukTM/ugjNz4CO2MjL4YzM)

**回调示例：**
```json
{ 
    "ts": "1502199207.7171419", //  事件发送的时间，一般近似于事件发生的时间。 
    "uuid": "bc447199585340d1f3728d26b1c0297a",  // 事件的唯一标识
    "token": "41a9425ea7df4536a7623e38fa321bae", // 即Verification Token 
    "type": "event_callback", // 此事件此处始终为event_callback
     "event": { 
         "type": "contact_scope_change", // 事件类型 
         "app_id": "cli_xxx",   // 应用ID
         "tenant_key": "xxx", //企业标识 
     } 
}
```
