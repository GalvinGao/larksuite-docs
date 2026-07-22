---
document_id: '6963854764275204101'
directory_id: '7262910572679708677'
title: 服务端通用错误码
full_path: /ukTMukTMukTM/ugjM14COyUjL4ITN
breadcrumb:
- Server API
- API Call Guide
- Server Error Codes
document_type: ReferenceDocumentType
updated_at: 2022-03-03T08:37:12Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ugjM14COyUjL4ITN
---

# 服务端错误码说明
>  以下是服务端常用错误码列表，部分未列入的错误码可以在具体 API 接口文档中查询到。你也可以通过右上角搜索功能，全局查找错误码说明和排查建议。

| 错误码 | 说明 | 排查建议 |
| --- | --- | --- |
| 1002 | Failed | 文档已删除，已删除文件无法获取文档内容 |
| 1500 | internal error | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 1503 | internal error | 内部错误，更新token，但无任何鉴权结果返回，请检查后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 1551 | internal error | 获取tenant id错误，如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 1557 | internal error | 获取open user错误，如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 1642 | internal error | 内部错误，更新session失败，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 1663 | internal error | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 1665 | internal error | 内部错误，请[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 1668 | internal error | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 2200 | internal error | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 4000 | bad request | 传参错误，请确保请求方法、请求信息、请求数据格式等是正确的，如富文本格式是否正确 |
| 4001 | Invalid token, please refresh | 查看token是否填写正确，是否过期 |
| 4002 | definition not found | 审批定义找不到，检查审批定义code是否正确 |
| 4003 | instance not found | 审批实例找不到，检查审批实例ID是否正确 |
| 4004 | user not found | 用户找不到，检查userId是否正确 |
| 4005 | method not allowed | HTTP 方法不支持，检查是否是POST 请求 |
| 4006 | service exception | 服务异常，检查服务可用性 |
| 5000 | internal error | 内部错误，减少调用频率，稍后再试 |
| 9499 | bad request 或 invalid param | 请求参数有误，请检查参数的正确性 |
| 10001 | Your request contains an invalid request parameter | 请求参数无效 |
| 10002 | Bot can NOT be out of the chat | 机器人不在会话中，请添加机器人后重新请求 |
| 10003 | invalid parameter | 请求参数缺失或者有误，更多错误信息请参考请求返回的error message |
| 10004 | user not found | 未找到指定用户 |
| 10005 | app id unauthorized | 请求鉴权失败，当前请求的应用信息或者租户信息有误 |
| 10009 | get open_chat_id fail | 获取 open chat id 失败 |
| 10010 | send message forbidden | 禁止跨企业发送消息 |
| 10012 | get app access token fail | 获取App Token失败，请参考 [API访问凭证概述](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| 10013 | get tenant access token fail | 获取Tenant Token失败，请参考 [API访问凭证概述](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| 10014 | app unauthorized | 应用状态不可用，检查应用是否停用 |
| 10015 | wrong app secret | App Secret 错误，请在开发者后台确认当前应用的App Secret |
| 10016 | app resend fail | 重新推送应用 ticket 失败，检查应用是否为ISV应用，自建应用无法调用该接口 |
| 10017 | Bot is NOT the owner of the resource | 机器人不是资源的所有者，无法获取资源详情 |
| 10019 | urgent type not support | 未知的加急类型，当前支持应用内加急、短信加急、电话加急三种类型 |
| 10020 | message id not exist | Message ID 非法 |
| 10023 | urgent scope unauthorized | 没有对应加急类型的权限，没有对应加急类型的权限。权限的开启需要提交版本发布，通过管理员审核后才能生效，详情参见[开启应用权限方法](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN#e601f785)。对开发测试阶段的应用可以使用“测试版应用”功能，申请权限无需发布版本，等待管理员审核，可直接在测试租户完成，详情参见[教程](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)。 |
| 10029 | chat_id not exist | Open Chat ID非法 |
| 10030 | bot not in chat | 机器人不在会话中，请添加机器人后重新请求 |
| 10032 | valid user is null | 请求参数中用户列表为空，请检查请求请求参数 |
| 10034 | chat tenant id mismatch tenant access token | 租户信息不匹配，不能跨企业发起请求 |
| 10037 | open_message_id not exist | Open Message ID非法 |
| 10100 | invalid request | 请求参数错误，检查请求参数是否合法 |
| 10101 | internal error | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 10105 | internal error | 内部错误，获取open user错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 10200 | crop failed | 图片裁剪失败，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 10204 | wrong code | code重复使用了或已过期 |
| 10400 | bad request | 客户端参数错误，一般不需要排查 |
| 10500 | internal server error | 服务端异常错误，建议服务端排查 |
| 11000 | get sso access token fail | 获取SSO Token失败 |
| 11001 | check security token fail | 校验Security Token 失败 |
| 11100 | check chat by open chat id fail | Open Chat ID非法 |
| 11101 | open_id not exist | Open User ID非法 |
| 11102 | get open_id fail | Open User ID非法 |
| 11103 | open department id not exist | Open Department ID非法 |
| 11104 | get open department id fail | Open Department ID非法 |
| 11105 | user_id not exist | Employee ID非法 |
| 11106 | get employee_id fail | 获取Employee ID失败 |
| 11200 | only admin can modify chat name | 更新会话信息失败，不是群主，无法更改群聊信息 |
| 11201 | bot is not chat owner | 机器人不在会话中，请添加机器人后发起请求 |
| 11202 | only admin can add chatter | 向会话添加成员失败，机器人不是群主，不能添加成员 |
| 11203 | send employee ids permission denied | 发送 employee id 信息失败，请确保在应用后台已经设置了获取Employee ID的选项 |
| 11204 | send department list permission denied | 发送部门列表失败，请确保在应用后台已经设置了获取Department ID的选项 |
| 11205 | app do not have bot | 应用没有开通机器人能力，请确保在应用后台开通了机器人能力 |
| 11206 | user not in chat | 用户不在会话中，请添加该用户再发起请求 |
| 11207 | app is not usage | 应用在当前租户下不可用，请在应用后台检查应用的状态 |
| 11208 | only group admin can remove chatter | 会话移除成员失败，机器人不是群主，不能移除群组成员 |
| 11209 | app not exist | 应用不存在 |
| 11210 | app usage info not exist | 应用在当前租户下未安装，不可使用，联系租户管理员，确认应用安装状态 |
| 11211 | add chatters is empty | 会话成员信息有误，会话添加成员列表为空 |
| 11212 | invalid chat_id or user_id | 删除会话成员信息失败，Chat ID或者User ID有误 |
| 11213 | chat not found | 指定的会话不存在 |
| 11214 | image not given | 上传图片操作失败，文件列表为空 |
| 11215 | chat_id is nil | Chat ID非法 |
| 11216 | get chat id fail | Chat ID非法 |
| 11217 | this group is set to only admin can add new member | 向会话添加成员失败，当前用户不是群主，不能像群组中添加成员 |
| 11218 | the number of robots exceeds the limit | 会话中机器人数量已经达到上限，不能继续添加机器人 |
| 11219 | cross tenant chat forbidden add bot | 跨企业会话无法添加机器人 |
| 11220 | only admin can disband group | 解散会话失败，当前用户不是群主，不能解散群组 |
| 11221 | this image does not belong to you | 获取图片数据失败，机器人没有权限获取不属于自身的图片 |
| 11222 | bot owner not in chat | 应用所有者不在会话中，不能添加机器人到会话 |
| 11223 | do not have im write permission | 当前操作者没有发送消息的权限，请开通权限后发起请求 |
| 11224 | message_id invalid | Message ID非法 |
| 11225 | the bot is invisible to the user | 机器人对用户不可见，不能对其进行操作。可在开发者后台-应用发布-版本管理与发布 编辑应用对用户的可见性并发布。 |
| 11226 | app_id is nil | App ID非法 |
| 11227 | image key not exist | Image Key非法 |
| 11228 | chat is not group | 当前会话不是群组类型 |
| 11229 | no permission | 请确保在应用后台已经开启了获取用户信息的权限 |
| 11230 | internal error | 机器人禁止发送当前消息类型的消息 |
| 11231 | chat not found | 请求的会话不存在 |
| 11232 | create message service trigger rate limit | 创建消息触发系统超限，请参考 [频控策略](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 11233 | create message chat trigger rate limit | 创建消息触发系统超限，请参考 [频控策略](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 11234 | this message do not belong you | 非机器人消息，机器人无权获取不属于自己的信息的读取状态信息 |
| 11235 | this group is set to only admin can mention @All | 当前会话禁止@所有人操作 |
| 11236 | user is resigned | 用户已经离职 |
| 11237 | group chat dissolved | 指定会话已经解散，不能添加成员 |
| 11238 | can not recall too old message | 发送较长时间的消息不能召回 |
| 11239 | permission denied | 尝试获取其他租户信息，操作失败 |
| 11240 | Robot not turned on | 机器人没有启用，请确保在应用后台开通了机器人能力 |
| 11241 | no user_info scope | 应用权限没有开启用户信息选项，请确保在应用后台已经设置了获取用户信息的选项 |
| 11242 | this chat has been banned | 当前会话已经被封禁，无法对会话内消息进行回复 |
| 11244 | file key not exist | 文件不存在 |
| 11245 | this file does not belong to you | 尝试获取不属于自己的文件，操作失败 |
| 11246 | create message content fail | 创建消息内容失败，更多错误信息请参考请求返回的error message |
| 11247 | internal send message trigger rate limit | 请求触发频控，请参考 [频控策略](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 11248 | message is sensitive | 消息内容涉及敏感信息，发送失败 |
| 11249 | bot not found | 给定的机器人不存在 |
| 11251 | chat name too short | 会话名字太短，确保不少于2个字符 |
| 11252 | chat information review fail | 会话信息审核未通过，确保会话信息不包含敏感信息 |
| 11310 | get card message fail | 消息卡片请求失败，更多错误信息请参考请求返回的code和message，以及卡片错误码文档 |
| 11311 | urgent message fail | 加急消息错误，仍然出现可以咨询客服 |
| 11312 | messages do not pass the audit | 消息内容不合法，请检查消息内容 |
| 12001 | Your request contains an invalid request parameter. | 请求参数无效 |
| 12002 | Non-chat-owner can NOT edit chat information in the current situation. | 群主设置"仅群主可编辑群信息"，非群主无法更新群信息 |
| 12008 | Your request specifies a chat whose type is NOT supported currently | 会话类型不支持 |
| 12011 | Operator can NOT be out of the chat. | 机器人或授权用户不在群组中，请添加后再发起请求 |
| 12015 | Your request specifies a member_id_type which is NOT supported. | 不支持member_id_type，仅支持以 open_id/user_id/union_id/app_id 作为查询参数 |
| 12016 | Non-chat-owner can only edit certain parts | 群主设置"仅群主可编辑群信息"，非群主无法更新群信息 |
| 12018 | Updating announcement failed | 更新公告失败 ，请查看日志了解具体原因 |
| 18017 | create urgent message fail | 创建加急消息失败，请确保请求参数正确 |
| 18033 | upload image fail | 上传图片失败，请确保请求参数正确 |
| 18034 | get chat id fail | 获取 chat id 失败，请确保请求参数正确 |
| 18035 | image_key require | 获取图片数据失败，请确保请求参数正确 |
| 18053 | create message fail | 创建消息失败，请确保请求参数正确 |
| 18054 | create message content fail | 创建消息内容失败，请仔细检查消息内容是否涉及敏感信息 |
| 18113 | thread not found | 检查当前话题是否存在 |
| 18117 | reach limit of chat size | 群人数超过上限 |
| 18121 | create request is being processed | 相同的创建消息请求正在处理中，请稍后重试。 |
| 19002 | incorrect input format or can't find msg_type | 参数格式不对或缺少msg_type参数，检查参数格式，并确认传入了msg_type |
| 19036 | The message exceeds the size limit of 30KB | 参数大小超过限制，检查消息体大小 |
| 20001 | invalid request | 无效的请求体，请确保请求方法、请求信息、请求数据格式等是正确的 |
| 20005 | invalid access_token | 无效的 access_token，请确保header里的 access_token 是正确、有效的，详情可参考[API访问凭证概述](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| 20007 | generate access_token fail | 生成 access_token 失败，请确保 code 没有重复消费或过期消费；小程序 tt.login() 获取的 code 不可用于服务端 API 获取用户信息，反之亦然 |
| 20009 | tenant app not installed | 应用在当前租户未安装 |
| 20021 | user resigned | 用户已离职，检查用户状态 |
| 20022 | user frozen | 用户已冻结，检查用户状态 |
| 20023 | user not registered | 用户未注册，检查用户状态 |
| 20024 | code app_id not match | appID不匹配，请确保生成code的appID与请求header里app access token对应的appID保持一致 |
| 30005 | interface is offline | 接口已下线，请切换到新版本接口 |
| 40001 | param error | 请求参数非法，请检查请求参数 (注：通讯录V3版本接口出现该错误码时取该含义) |
| 40001 | market place app is not allowed to update user or department | 应用商店应用不允许修改通讯录，请停止尝试 |
| 40002 | process root dept error | 不能对顶级部门（根部门）进行操作，请检查是否在请求参数的部门ID中填入0 |
| 40003 | no department authority | 应用没有对应的部门的通讯录授权，请检查通讯录授权情况；如需要访问相应部门，请联系管理员添加相应部门的通讯录授权 |
| 40003 | internal error | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)  （注：通讯录V3版本接口出现该错误码时取该含义） |
| 40004 | no dept authority error | 应用没有部门权限，检查该部门是否在通讯录权限范围内 |
| 40006 | internal error | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 40007 | user_id %v is not exist | 提供的用户ID不存在，请检查用户ID来源，是否为当前租户下用户 |
| 40008 | deptInfo is null error | 部门信息为空。可能系统异常导致部门不存在，请[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)   (注：通讯录V3版本接口出现该错误码时取该含义) |
| 40008 | open_id %v is not exist | 提供的用户open_id不存在，请检查用户open_id来源 |
| 40009 | page size is more more than 50 | 分页的大小超出50这个最大限制，请检查page_size设置的大小 |
| 40010 | chat id is invalid error | 部门群ID非法 |
| 40011 | page size is invalid error | 分页请求的page_size参数非法，请检查page_size设置的大小 |
| 40012 | page token is invalid error | 分页的page_token非法，请检查参数合法性(注：通讯录V3版本接口出现该错误码时取该含义) |
| 40012 | no parent dept authority error | 应用没有父部门权限，检查该部门是否在通讯录权限范围内 |
| 40013 | param error | 参数错误，具体错误原因可参考message信息，如有疑问，请联系管理员 |
| 40014 | dept name can not be nul error | 部门名不能为空，请检查请求中的部门名设置 （注：通讯录V1、V2版本接口出现该错误码时取该含义） |
| 40014 | no parent dept authority error | 没有父部门权限，检查该部门是否在通讯录权限范围内 (注：通讯录V3版本接口出现该错误码时取该含义） |
| 40016 | name can not be null in updateRequest error | 更新接口，需带上name字段 |
| 40018 | duplicate name error | 部门名字冲突，同一父部门的子部门间名字不能相同 |
| 40021 | not a same request error | client_token判断非同一个请求。两次操作不是同一个请求，请检查request值是否有改动(注：通讯录V3版本接口出现该错误码时取该含义) |
| 40021 | forbid update department id | 禁止修改部门自定义ID，部门自定义ID是部门当前的唯一ID，企业有其他应用使用通讯录情况下禁止修改 |
| 40031 | task_id is not exist | task_id 不存在，请检查task_id 来源 |
| 40032 | task_id invalid | task_id非法，请检查来源，是否为当前企业上传任务生成的task_id |
| 40040 | invalid page_token | 分页请求的page_token参数非法，请检查page_token来源 |
| 40041 | request page_size invalid | 分页请求数量不符合接口要求，请参考接口文档传递正确的参数值 |
| 40042 | batch request object count error | 批量接口请求数量不符合接口要求，请参考接口文档传递正确的参数值 |
| 40051 | open_id %v is not valid | 用户的open_id非法，检查用户open_id来源，是否为当前应用下的open_id |
| 40052 | department id %v is not exist | 部门ID不存在，请检查部门ID来源 |
| 40053 | role_id is not exist | 角色ID不存在，请检查角色ID来源 |
| 40054 | user_id or open_id is needed | 没有有效的用户ID，user_id或者open_id至少提供其中之一 |
| 40101 | mobile is already exist | 手机号已存在，手机号在企业内必须唯一，手机号在该企业创建过用户不允许再创建新用户 |
| 40102 | email is already exist | 邮箱已存在，邮箱在企业内必须唯一，邮箱在该企业创建过用户不允许再创建新用户 |
| 40103 | mobile and email cp info not match | 手机号和邮箱的账号信息冲突，用户的联系方式属于两个不同的Lark账号，添加失败。如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 40104 | mobile is not valid | 手机号不合法，请检查是否是正确的手机号格式 |
| 40105 | name can’t be null | 部门、用户等name字段不允许为空 |
| 40106 | email is not valid | 邮箱格式不正确，请检查邮箱地址的有效性 |
| 40107 | user count exceed certification tenant limit | 用户数量超过未认证企业的数量限制，不能创建用户，如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 40108 | user count exceed bill limit | 用户数量超过当前企业套餐的数量限制，不能创建用户，如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 40109 | neither email nor mobile is not provided | 邮箱手机号都未指定，创建用户，手机号邮箱至少指定其一 |
| 40110 | mobile is required | 创建用户，手机号必填 |
| 40111 | user_id is already exist | user_id 已存在，user_id是企业内用户的唯一ID，不能重复 |
| 40112 | user_id %v is not vaild | 创建用户的user_id格式不合法 |
| 40113 | must assign department for user | 未指定部门，创建用户，必须指定在哪个部门下创建用户 |
| 40114 | name contains sensitive words | name中包含敏感信息，不允许创建，如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 40115 | invalid join_time | 用户入职时间不合法 |
| 40116 | invalid gender | 性别参数不合法，取值请参考文档接口参数说明 |
| 40117 | idp_type is required | idp企业创建用户，idp_type字段必填 |
| 40118 | idp_type is invalid | idp_type类型非法，请填入正确的idp_type |
| 40119 | support set or update user_id at most once | 用户自定义ID只允许设置或者更新一次。用户的自定义ID，创建时未指定自定义ID允许修改一次；创建时指定了自定义ID不再允许二次修改 |
| 40120 | custom_attr id is not set | 自定义属性ID未指明，设置用户自定义属性，必须指明设定的属性ID，属性ID可以通过获取企业自定义属性接口查询 |
| 40121 | custom_attr %v value not set | 自定义属性值未设定。设置自定义属性，需要传入属性value字段 |
| 40122 | custom_attr id %v is not exist | 自定义属性ID不存在，请确认自定义属性ID来源，自定义属性ID可以通过获取企业自定义属性接口查询 |
| 40123 | href custom_attr %v text can’t be null | 设置HREF类型自定义属性，text字段为必填字段 |
| 40124 | href custom_attr %v url can’t be null | 设置HREF类型自定义属性，url字段为必填字段 |
| 40125 | user position info invalid | 创建用户岗位信息非法。创建用户岗位时，岗位名称、岗位码和部门都不能为空，有岗位leader时，必须同时指定leader的ID信息 |
| 40126 | position department invalid, should be one of user’s department | 设置用户岗位部门ID不合法，用户的岗位部门ID必须为用户所在的部门之一 |
| 40127 | position code is not unique | 用户的岗位码不唯一，同一个用户的多个岗位岗位码信息必须唯一 |
| 40128 | user has one main position at most | 用户主岗数量错误，一个用户至多只能有设置一个主岗 |
| 40129 | request contains users of different tenant | 请求的用户来自不同企业，检查请求的用户ID来源，是否为同一企业的用户 |
| 40130 | update user department conflict with position department, should update position department at the same time | 更新用户的部门与用户岗位部门冲突，用户的岗位部门必须与用户的部门一致，更新用户部门需要同时更新相应的岗位部门，否则阻断更新操作 |
| 40131 | update user position department conflict with user department, should update user department at the same time | 更新用户的岗位部门与用户部门冲突，用户的岗位部门必须与用户的部门一致，用户的新岗位部门也必须是用户的部门之一，否则阻断更新操作 |
| 40132 | order department invalid | 用户排序的部门ID非法，请求的用户排序信息中的部门ID必须是用户的部门ID之一 |
| 40133 | idp check with account failed | IDP 账号检查失败 |
| 40134 | create account failed | 账号创建失败 |
| 40135 | idp user_id is required | 创建IDP账号，user_id 必填 |
| 40136 | update user’s email and mobile empty is forbiden | 更新用户信息，不能将用户的手机号和邮箱同时更新为空 |
| 40137 | can’t update unactive user’s mobile empty | 更新用户信息，不能将未激活用户的手机号更新为空 |
| 40138 | %v is not valid | 接收者非法。用户离职，用户资源接收者必须为当前企业的在职用户 |
| 40139 | unable to join multiple departments, please upgrade Organizational Structure Visible info | 用户无法加入多部门，请在企业管理后台升级“组织架构可见范围”规则 |
| 40140 | can not set leader to oneself | 不能设置用户的直属上级为其本身，请检查用户的直属上级参数值 |
| 40141 | can not set user position | 当前企业不支持设置用户岗位信息，如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 40142 | user department num exceed the limit | 不支持用户同时属于50个以上的部门，请检查 |
| 40143 | unable to join multiple departments, please enable multiple departments feature | 当前企业不支持用户同时加入多个部门，如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 40144 | department user count exceed | 创建或更新的所属部门中至少有一个部门的直属成员数超过限制，请检查请求中部门的人数 |
| 40151 | custom department id %v is invaild | 创建或者更新部门，自定义部门ID非法 |
| 40152 | custom department id %v is not unique | 自定义部门ID已存在，同一企业内，自定义部门ID必须唯一 |
| 40153 | department name should be unique under same department | 部门名称冲突。同一部门下，部门的名称不允许重复 |
| 40154 | department unit %v is not exist | 部门单元不存在，请检查部门单元ID |
| 40155 | duplicate department order | 部门排序order重复。同一部门下，部门order不允许重复 |
| 40156 | department id is required | 请求未指定部门ID |
| 40157 | forbidden operation to root department 0 | 不允许操作根部门0。根部门是虚拟部门，不能查询部门请求，更新该部门或者删除该部门 |
| 40158 | support update custom department id at most once | 超过自定义部门ID更新限制，自定义部门ID至多允许更新一次 |
| 40159 | department has active members, can’t delete | 待删除部门有在职用户时，不允许直接删除部门，请先处理部门成员后再执行删除操作 |
| 40160 | department has sub department, can’t delete | 待删除部门有子部门时，不允许直接删除部门，请先删除子部门后再删除部门 |
| 40161 | can’t get root department info | 根部门为虚拟部门，不能查询其详情信息 |
| 40162 | departments with more than 500 sub departments are not allowed to call recursively | 当部门的子部门数量超过500个时，不支持递归查询该部门的子部门或者递归查询该部门的成员。请自行实现递归查询逻辑 |
| 40163 | can not create department unit | 当前企业不支持创建部门单元，如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 40164 | department child count exceed | 创建或更新的部门，其所属父部门的子部门数量超过限制，请检查该部门的子部门数量 |
| 40170 | unit repeat, unit_type and unit_name is exist, can’t create | 部门单元重复。同一企业下，同一部门单元类型下name必须唯一 |
| 40171 | department unit name invalid | 部门单元名称非法，详见接口文档参数描述 |
| 40172 | department unit type invalid | 部门单元类型非法，详见接口文档参数描述 |
| 40173 | custom_id is already exist | 部门单元自定义ID已存在。同一企业内，自定义部门单元ID必须唯一 |
| 40174 | department unit is still using, can’t be deleted | 部门单元正在使用中，例如有其他部门关联到了该部门单元，此时不允许删除部门单元 |
| 40175 | department attach to more than one department unit | 一个部门最多关联一个部门单元，无法关联多个部门单元 |
| 40180 | duplicate user_id in request | 批量请求中，user_id有重复。user_id是用户在企业的唯一标示，不允许重复；请检查批量请求参数 |
| 40181 | add user, leader must not user himself | 创建用户，用户的直属上级不能为用户自己 |
| 40182 | MSyncUser must user_id must be assigned | 批量同步用户，必须指定用户的user_id |
| 40183 | leader user_id circular reference | 批量同步用户，leader信息成环；比如同一个请求创建用户A、B，指定A的leader为B，同时指定B的leader为A的场景。请检查请求参数 |
| 40184 | add department, parent_id must not equal to current department id | 批量创建部门，部门的父部门不能为部门本身 |
| 40185 | add department, parent_id must be assigned | 创建部门，必须指定父部门 |
| 40186 | duplicate department id in request | 批量创建部门，请求中部门自定义ID重复，请检查请求参数 |
| 40187 | batch request, parent department id incompatible | 批量创建部门，部门参数出现冲突 |
| 40501 | param required | 检查必填的参数 |
| 40502 | invalid parameter | 请求参数缺失或者有误，更多错误信息请参考请求返回的error message |
| 41001 | mobile already exist error | 手机号已经在企业中存在。手机号在企业内必须唯一，手机号在该企业创建过用户不允许再创建新用户 |
| 41002 | email already exist error | 邮箱已经在企业中存在。邮箱在企业内必须唯一，邮箱在该企业创建过用户不允许再创建新用户 |
| 41003 | user acount conflict error | 用户的联系方式属于两个不同的Lark账号，添加失败。如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 41004 | mobile is invalid error | 手机号不合法，请检查是否是正确的手机号格式 |
| 41005 | email is invalid error | 不是合法邮箱的邮箱地址，请检查邮箱地址的有效性 |
| 41006 | no user name error | 无效的用户名，请检查是否传入了用户名 |
| 41007 | exceed uncertain tenant seat limit error | 用户数量超过未认证企业的数量限制，不能创建用户，如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 41008 | exceed bill seat limit error | 用户数量超过未认证企业的数量限制，不能创建用户，如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 41009 | no email or mobile error | 创建用户，手机号邮箱至少指定其一 |
| 41010 | no mobile error | 创建用户，手机号必填 |
| 41011 | userID already exist error | 用户id已存在。user_id是企业内用户的唯一ID，不能重复 |
| 41012 | user id is invalid error | 用户id非法。user_id1-64字节的之间的字符 |
| 41013 | exceed userID update limit error | 超出更新userID的次数上限 |
| 41014 | user name sensitive error | 用户名中包含敏感信息，如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 41015 | idp type invalid error | 登录类型无效 |
| 41016 | department has too may users error | 部门下成员数量过多。部门下直属成员有数量限制，超过限制不允许添加 |
| 41017 | department not exist | 部门不存在 |
| 41018 | position info is invalid error | 岗位信息无效 |
| 41019 | position department is invalid error | 岗位部门无效，查看部门信息 |
| 41020 | position code has already exist error | 岗位code无效 |
| 41021 | position multiple main count error | 用户主岗错误，一个用户至多只能有设置一个主岗 |
| 41022 | user tenant not match error | 用户的企业信息和应用不对应，检查是否使用其他企业的凭证访问当前企业的资源 |
| 41023 | update department conflict position department error | 用户的岗位部门与用户的部门不一致。用户的岗位部门必须与用户的部门一致，更新用户部门需要同时更新相应的岗位部门，否则阻断更新操作 |
| 41024 | update position department conflict department error | 用户的岗位部门与用户的部门不一致。用户的岗位部门必须与用户的部门一致，更新用户部门需要同时更新相应的岗位部门，否则阻断更新操作 |
| 41025 | order department invalid error | 部门order非法，请求的用户排序信息中的部门ID必须是用户的部门ID之一 |
| 41027 | create account failed error | 创建用户失败，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 41028 | user multi department need upgrade visibility error | 使用多部门需要将可见性版本升级，可在企业管理后台-组织架构可见范围中升级，如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 41029 | create or update user multi department error | 更新用户多部门失败。可能多部门功能未开启，咨询企业管理员 |
| 41030 | set leader to oneself error | 不允许设置自己的上级为自己 |
| 41031 | position feature not enable error | 岗位设置开关未打开。当前企业不支持设置用户岗位信息，如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 41032 | user multi department feature not enable error | 允许用户加入多部门开关未打开。当前企业不支持用户同时加入多个部门，如有疑问可[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 41033 | user in too many departments error | 用户加入的部门数过多。不支持用户同时属于50个以上的部门，请检查 |
| 41034 | email prefix already exist error | 电子邮件的前缀已经存在 |
| 41035 | email prefix is invalid error | 电子邮件的前缀不合法，请检查拼写 |
| 41036 | avatar key is invalid error | 头像非法，头像的key通过上传接口获取 |
| 41037 | avatar is sensitive error | 头像的涉及敏感信息，不可用 |
| 41038 | gender is invalid error | 性别不合法，申请自查 |
| 41040 | user name is null error | 用户名为空，请检查参数 |
| 41041 | department id is not assigned error | 用户所属的部门不能为空 |
| 41042 | join time is invalid error | 用户的加入时间非法，加入时间应该是有效的时间戳 |
| 41043 | employee id is not valid error | employee id 非法，employee id 长度应该在1-64个字符之间，不应包含\t\n\r |
| 41044 | custom attr id is not set error | 请求参数的自定字段ID为空 |
| 41045 | Custom attribute id is not exist error | 自定义属性ID不存在，请确认自定义属性ID来源，自定义属性ID可以通过获取企业自定义属性接口查询 |
| 41046 | custom attr value is not set error | 自定义属性对应的值为空 |
| 41047 | custom attribute href text is null error | HREF类型自定义属性， herf text字段为必填字段 |
| 41048 | custom attr href url is null error | HREF类型自定义属性,url为必填字段 |
| 41050 | no user authority error | 没有用户权限，检查该用户是否在通讯录权限范围内 |
| 41051 | user id info not provide error | 请求user id 为空，请检查user id 参数 |
| 41052 | acceptor id is invalid error | 辞职资源接受人无效。用户离职后，其文件的接收者不合法 |
| 41053 | userID already exist error | 用户id已存在。user_id是企业内用户的唯一ID，不能重复 |
| 41055 | employee type can not be null error | 更新接口中，雇员类型不可为空，请检查 |
| 41056 | no field authority error | 请求中存在无权限的字段，请查看权限范围 |
| 41057 | need send email but not set mail error | 需要发送邮件，但是没有设置邮箱 |
| 41058 | need send sms but not set mobile error | 需要发送短信，但是没有设置手机号 |
| 41059 | invalid employee type error | 用户的雇员类型错误，请填写1-5之间的数字，1 正式员工 2 实习生 3 外包 4 劳务 5 顾问 |
| 41060 | inactive employee type error | 员工类型未启用。当前企业并无相应的雇员类型，企业的雇员类型咨询管理员 |
| 42005 | user has exist in group error | 用户已经存在于用户组 |
| 42006 | user has resigned error | 用户已经离职 |
| 42008 | invalid tenant id error | 非法的租户ID，请自查租户是否可用 |
| 43004 | illegal unit error | 非法的单位信息，确保单位信息合法有效 |
| 43005 | duplicate order error | 重复的排序，同一部门中order必须唯一 |
| 43007 | dup dept unit custom id error | 租户内 部门单位自定义ID (unit_id) 重复，或部门自定义ID (department_id) 重复 |
| 43008 | custom dept id invalid error | 部门自定义ID不合法 |
| 43010 | big dept forbid recursion error | 部门的子部门过多，不支持递归查询 |
| 43011 | delete has member dept error | 删除有用户的部门出错 |
| 43013 | dept too many children error | 部门的直接子部门数量不超过1000 |
| 45500 | internal error | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 47009 | duplicate name error | 名字重复，更换用户组名称重试 |
| 50003 | invalid app_id | 传参错误，请确保app_id是正确、有效的。 |
| 50006 | invalid request | 请求参数错误，建议检查请求参数是否合法 |
| 55001 | server internal error | 服务内部错误，减少调用频率，稍后再试 |
| 60001 | OpenApiErrorCodeParameterError | 请求参数错误，检查请求参数 |
| 60002 | OpenApiErrorCodeDefinitionNotFound | 审批定义【approval_code】 找不到，检查审批定义code是否正确 |
| 60003 | OpenApiErrorCodeInstanceNotFound | 审批实例【instance_code】找不到，检查审批实例ID是否正确 |
| 60004 | OpenApiErrorCodeUserNotFound | 用户找不到，检查userId是否正确 |
| 60005 | OpenApiErrorCodeDepartmentNotFound | 部门验证失败，检查用户所属部门 |
| 60006 | OpenApiErrorCodeValidateFormError | 表单验证失败，检查表单权限 |
| 60007 | OpenApiErrorCodeSubscriptionExist | 订阅已存在，检查已订阅列表 |
| 60008 | OpenApiErrorCodeSubscriptionNotExist | 订阅不存在，检查是否已订阅，未订阅不可取消 |
| 60009 | OpenApiErrorCodeNoPermission | 权限不足，检查权限 |
| 60010 | OpenApiErrorCodeTaskNotFound | 审批任务【task_id】 找不到，确认taskId是否正确 |
| 60011 | OpenApiErrorCodeStartPremiumVersion | 该审批为付费审批，免费版用户不能发起这个审批，付费审批免费用户不可发起 |
| 60012 | OpenApiErrorCodeUuidConflict | 审批实例 uuid 冲突，uuid需要保持唯一 |
| 60013 | OpenApiErrorCodeUnsupportApporval | 不支持的审批定义，免费用户不支持该类型审批 |
| 60014 | 获取更新锁失败 | 并发更新同一个审批实例冲突。 |
| 65001 | OpenApiErrorCodeInternalError | 内部错误，建议检查一下参数并重试 |
| 90201 | WrongRequestJson | 请求体不是一个 json，请确保请求body是合法的json格式 |
| 90202 | WrongRange | 请求中range格式有误，检查请求中的range是否合规 |
| 90203 | Fail | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 90204 | WrongRequestBody | 请求体有误，请求入参有问题，请仔细检查入参 |
| 90205 | InvalidUsers | 非法的 user，检查user信息是否正确 |
| 90206 | EmptySheetId | sheeId 为空，必须要提供sheetid，修正请求入参 |
| 90207 | EmptySheetTitle | sheet 名称为空，检查sheet名称是否正确 |
| 90208 | SameSheetIdOrTitle | 请求中有相同的 sheetId 或 title，调整sheetId或者title |
| 90209 | ExistSheetId | sheetId 已经存在，请提供不重复的sheetId |
| 90210 | ExistSheetTitle | sheet title 已经存在，请提供不重复的子表title |
| 90211 | WrongSheetId | 错误的 sheetId，检查sheetId的正确性 |
| 90212 | WrongRowOrCol | 非法的行列，检查行列信息是否正确 |
| 90213 | PermissionFail | 没有文件的权限 forbidden，请获取文件权限 |
| 90214 | SpreadSheetNotFound | SpreadSheet没有找到，请检查spreadsheet token |
| 90215 | SheetIdNotFound | sheetId 没有找到，请求入参对应的子表不存在，修正请求参数 |
| 90216 | EmptyValue | 请求参数错误 或 本次请求未导致数据变更，请检查参数 |
| 90217 | TooManyRequest | 请求太频繁，请降低请求调用频率 |
| 90218 | LockedCell | 单元格被保护，检查是否有权限编辑该单元格 |
| 90219 | CellExcess | 单元格数量超过限制，检查单元格数量 |
| 90221 | TooLargeResponse | 需要返回的数据过大，降低查询量 |
| 90222 | TooLargeCell | 单元格内容过大，减少单元格内容 |
| 90223 | ColIdNotFound | ColId没有设置，检查请求体是否设置ColId字段 |
| 90224 | RowIdNotFound | RowId没有设置，检查请求体是否设置RowId字段 |
| 90225 | NotLinkSpreadSheet | 未关联ISV |
| 90226 | ExcessLimit | 超出限制，根据返回的消息调整请求数量 |
| 90227 | TooLargeRequest | 请求过大，降低请求量 |
| 90228 | ImportProcessing | 导入中，等待导入完成 |
| 90229 | WrongURLParam | URL参数错误 |
| 90235 | Retry later | 服务端繁忙，请稍后重试；如果仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 90242 | Spreadsheet in mix state | 正在加载文档数据，请等待文档数据加载完成后重试 |
| 90301 | FAILED | 操作失败 |
| 90302 | PARAM_ERROR | 参数错误 |
| 90303 | FORBIDDEN | 没有权限 |
| 90304 | META_DELETED | 文件已删除 |
| 90305 | META_NOT_EXIST | 文件不存在 |
| 90306 | REVIEW_NOT_PASS | 评论内容审核不通过 |
| 90399 | INTERNAL_ERROR | 内部错误，详情请[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 91001 | OPEN_CODE_PARAM_ERROR | 参数错误，对照开发者文档，检查请求参数 |
| 91002 | OPEN_CODE_FORBIDDEN | 没有权限，操作者给其他人加权限前要先有权限 |
| 91003 | OPEN_CODE_INVALID_OPERATION | 操作异常，可能的原因有文档设置了不允许跨租户分享、协作者达到上限等 |
| 91004 | OPEN_CODE_USER_NO_SHARE_PERM | 用户没有共享权限，检查用户是否有分享权限 |
| 91005 | Resource is deleted | 资源已被删除 |
| 91201 | FAILED | 处理失败，稍后重试 |
| 91202 | PARAMERR | 参数错误，检查参数是否正确 |
| 91203 | NOTEXIST | 数据不存在，检查参数是否正确 |
| 91204 | FORBIDDEN | 检查用户对文档、文件夹的权限 |
| 91205 | DELETED | 数据已被删除，检查数据是否还存在 |
| 91206 | OUT_OF_LIMIT | 超过限制 |
| 91207 | DUPLICATE | 重复记录 |
| 91208 | REVIEW | 内容审查不通过 |
| 91401 | PARAMERR | 参数出现错误，检查参数有效性 |
| 91402 | NOTEXIST | 未找到，检查token是否有效 |
| 91403 | FORBIDDEN | 没有权限，检查是否有文档读权限 |
| 91404 | LOGIN_REQUIRED | 需要登录 |
| 93001 | param err | 入参校验不通过，检查token和id等信息是否正确 |
| 93002 | out of limit | 请求列表数量超出预设范围，page list控制在500以内 |
| 93003 | invalid user | 无效用户，检查用户是否存在 |
| 93004 | not found | node或space没有记录，检查nodeId和spaceId是否正确 |
| 93006 | internal err | 程序内部错误可以找研发排查具体问题，可直接找wiki研发确认 |
| 95001 | internal error | 内部错误，请稍后重试 |
| 95003 | internal error | 内部错误，请稍后重试 |
| 95005 | internal error | 内部错误，请稍后重试 |
| 95006 | Failed | 文档未找到，检查token是否有效 |
| 95007 | Failed | 文档已删除，已删除文件无法获取文档meta信息 |
| 95008 | internal error | 内部错误，请稍后重试 |
| 95009 | Failed | 没有权限，检查是否有文档读权限。[添加文档权限](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/create) |
| 95010 | internal error | 内部错误，请稍后重试 |
| 95011 | internal error | 内部错误，请稍后重试 |
| 95013 | Failed | 挂载文档失败，无效的folderToken或对目录无权限 |
| 95017 | 具体错误信息 | 读取文档内容失败，检查revison是否正确 |
| 95018 | 具体错误信息 | 解析文档内容失败，详见具体错误信息 |
| 95020 | 具体错误信息 | 批量更新文档操作失败，详见具体错误信息 |
| 95023 | revision too old | 版本号太老，请使用最新版本号 |
| 95024 | Failed | 参数无效，检查参数有效性 |
| 95025 | Failed | 解析请求失败，检查请求是否合法json |
| 95026 | Failed | 无效的batch_update 操作类型，检查batch_update接口request_type |
| 95050 | 具体错误信息 | 保存文档内容失败，详见具体错误信息 |
| 95051 | 具体错误信息 | 创建文档失败，详见具体错误信息 |
| 95201 | InternalError | 内部错误，详情请[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 95202 | InternalError | 内部错误，详情请[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 95203 | InternalError | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 95204 | InternalError | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 95205 | InternalError | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 95206 | InternalError | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 95207 | InternalError | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 95208 | InternalError | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 95209 | InternalError | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 95299 | DefaultCode | 其他错误，详情请[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 96001 | OPEN_CODE_INTERNAL_ERROR | 内部错误，先重试并检查请求参数，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 96002 | concurrency error,please retry | 权限并发编辑失败，重试请求，且不要并发操作同一篇文档的权限 |
| 96201 | LOCK | 内部错误，勿并发调用接口，请串行调用 |
| 96202 | RECOVER | 内部错误，详情请[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 96401 | FAILED | 失败，详见具体错误信息 |
| 96402 | TIMEOUT | 超时 |
| 96403 | PROCESSING | 请求正在处理中 |
| 100001 | page token invalid | page token 格式非法，请检查token参数 |
| 100002 | Invalid field selection [illegal field] | fields 中存在非法字段名，检查fields中的字段名称 |
| 100003 | time format must follow RFC3339 standard | 时间格式未遵循 RFC3339 标准，请检查时间格式 |
| 100004 | building id invalid | building ID 非法，请检查Building ID参数 |
| 100005 | room id invalid | room ID 非法，请检查roomID参数 |
| 105001 | internal error | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 105002 | invalid request | 无效请求，检查请求参数 |
| 121004 | data not exist | 无效的请求体，请确保请求方法、请求信息、请求数据格式等是正确的 |
| 154000 | bad request | 传参错误，请确保请求方法、请求信息、请求数据格式等是正确的 |
| 154001 | unauthorized | 请求鉴权失败，请检查auth信息是否正确，应用租户和服务台租户是否一致 |
| 190002 | invalid parameters in request | 参数无效，请确保参数名称、参数值、必须参数齐全等是正确的 |
| 190003 | internal service error | 内部错误，请稍后重试，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 190005 | app rate limited | 应用被限流，稍后再试，适当减小请求频率 |
| 191001 | invalid calendar_id | calendar_id无效，检查calendar id是否填写正确 |
| 191002 | no calendar access_role | 没有日历的访问权限。确保有日历从访问权限，检查一下对日历对访问角色 |
| 193001 | event not found | 日程没有找到，检查event id是否正确，event是否已创建 |
| 193003 | event is deleted | 日程已经被删除，检查event id是否正确 |
| 230003 | invalid_page_token | 无效的 page token，需要使用接口返回的 page token。确认page token 信息，需要使用接口返回的 page token |
| 230016 | the_message_was_sent_too_long | 消息发送时间已超过七天，无法查询。只能查询发送七天以内的消息 |
| 232002 | only_chat_owner_can_edit | 群主设置"仅群主可编辑群信息"，非群主无法更新群信息 |
| 232011 | operator_not_in_chat | 机器人或授权用户不在群组中，请添加后再发起请求 |
| 232014 | operator_has_no_permission | 操作者没有权限。仅群主或机器人是群的创建者且具备 更新应用创建的群信息 权限时，可以移除用户或者机器人 |
| 232016 | non_owner_can_only_edit_certain_parts | 机器人或授权用户非群主，仅可修改部分信息(avatar, name, description, i18n_names) |
| 232019 | The request has been rate limited. | 触发群限流，请控制请求速度 |
| 1000001 | unexpected head in request | 请求头错误，详情请[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 1000002 | invalid parameters in request | 参数错误，排查参数是否符合API预期 |
| 1000003 | internal service error | 服务器错误，详情请[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 1000004 | method rate limited | 接口请求过快，超出频率限制，降低请求频率 |
| 1000005 | app rate limited | 应用被限流，降低请求频率 |
| 1000007 | app bot_id not found | bot id没有被找到，检查请求应用是否存在 |
| 1001001 | invalid calendar_id | calendar id无效，检查calendar id是否正确 |
| 1001002 | no calendar access_role | 没有日历从访问权限，检查相应calendar id是否正确，相应日历是否有权限 |
| 1001004 | invalid calendar type | 日历类型无效，检查calendar id是否正确 |
| 1001501 | user not found | 用户不存在，排查是否传递了不正确的open_id |
| 1001502 | okr data not found | OKR数据不存在，排查是否传递了不正确的okr_id |
| 1001901 | internal server error | OKR内部错误，详情请[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 1001902 | no permission | OKR无权限访问，排查是否传递了token、okr_id |
| 1001903 | invalid parameters | 请求参数错误，排查OKR接口参数格式是否正确 |
| 1001999 | system exception | 未知错误，详情请[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 1002000 | acl not found | 访问控制不存在，检查acl_id是否正确，是否已正确创建 |
| 1003001 | event not found | 日程没有找到，检查日程id是否正确 |
| 1003003 | event is deleted | 日程已经删除，检查日程id是否正确 |
| 1004000 | attendee not found | 参与人没有找到，检查attendee id是否正确（标识attendee的唯一id） |
| 1004004 | invalid attendee type | 无效的参与人类型，检查参与人类型是否都是正确的 |
| 1050001 | TIME_CHECK_NOT_VALID | 时间范围填写错误，填写准确的时间范围，起始时间不得早于半年前，时间选择范围不能超过一个月 |
| 1050002 | ErrCode_DATABASE_ERR | 服务内部错误，请稍后重试 |
| 1050004 | Error_Param_Error | 参数填写非法，检查参数是否填写错误或者缺失 |
| 1050005 | Error_Page_Size_Invalid | 分页请求的page_size参数非法，请检查page_size设置的大小 |
| 1050006 | Error_Page_Token_Invalid | 非法的 page token，检查page token是否填写错误 |
| 1050007 | Error_Event_Name_Not_Found | 非法的 event name，检查event name是否填写错误 |
| 1050008 | Error_Open_Platform_RPC | 请稍后重试 |
| 1050009 | Error_Lark_ID_Not_Found | 提供的用户ID不存在，请检查用户ID来源，是否为当前租户下用户 |
| 1050011 | Error_Event_Module_Invalid | 事件模块非法，检查event module是否填写错误 |
| 1050012 | Error_Event_Module_Not_Match_Event_Name | 提供的事件类型和事件模块不一致，请修改事件类型或者事件模块保证两者一致 |
| 1050019 | Error_Not_Support_Query | 不支持的查询，检查是否添加了不支持的查询字段 |
| 1051002 | param validate error | 参数错误。时间范围不能超出限制，具体限制详见文档，并且时间格式要正确。page_size和page_token也要正确填写 |
| 1061041 | parent node has been deleted | 父节点已经被删除，请确认parent_token的节点是否被删除 |
| 1069301 | fail | 操作失败，重试，稳定失败建议[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 1069302 | param error | 参数错误，检查参数有效性 |
| 1069303 | forbidden | 没有权限，检查是否有待评论文档的评论权限（普遍文档默认是有阅读权限即有评论权限） |
| 1069304 | docs had been deleted | 云文档已被删除，检查待评论文档是否已被删除 |
| 1069305 | docs not exist | 云文档不存在，检查待评论文档是否能正常访问 |
| 1069306 | content review not pass | 评论内容审核不通过，排查评论内容是否存在不合法内容 |
| 1069307 | not exist | 不存在，检查待评论文档是否能正常访问、评论可能不存在/被删除 |
| 1069308 | exceeded limit | 超过数量上限限制，评论数据超过上限限制，详情请[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 1069399 | internal error | 内部错误，请重试，稳定失败建议[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 1103003 | tenant no idp config | 租户未开启SSO登录，检查SSO相关配置，确认无误后开启租户SSO开关 |
| 1103008 | idp user not bind idp cp | 当前用户未绑定企业级联合登录凭证，请调用绑定接口绑定企业级联合登录凭证 |
| 99991201 | resource not find | 请求路径错误(404)，检查路径是否填写正确 |
| 99991300 | invalid next parameter:%s | callback url中next参数有问题，请检查并携带正确的next参数 |
| 99991301 | request method doesn’t match | 请求方法的类型与接口设置的不一致，检查接口设置的请求方法(POST/GET/…)与请求时使用的是否一致 |
| 99991400 | request trigger frequency limit | 请求过于频繁，请降低请求频次 |
| 99991401 | ip %s is denied by app setting | IP 被白名单限制。请检查应用是否开启了 IP 白名单，如果开启了，仅白名单中的来源请求可以正常调用。<br>可以通过以下方式获取你的服务器出口 IP ：<br>1. 请咨询企业内IT部门的运维人员或云服务提供商；<br>2. 登录你的服务器，用命令行执行：```curl ifconfig.me``` 或 ```curl cip.cc``` 或 ```curl myip.ipip.net``` 或 ```curl ipinfo.io```。<br>获取到正确的服务器出口 IP 后，可以参考[这篇文档](/document/ukTMukTMukTM/ucTMxYjL3ETM24yNxEjN)，修改你应用的 IP 白名单设置。 |
| 99991501 | auth login invalid code | 无登录code，请重新登录，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 99991503 | invalid next parameter: invalid host | callback url中next参数有问题，next和callback的host不相同 |
| 99991543 | login_user invalid | app_id或app_secret不存在，请检查参数 |
| 99991611 | get session fail | 小程序请求获取session失败，请检查请求是否带了相应的cookie(lobsession) |
| 99991612 | session invlaid | 小程序请求鉴权失败，请求的session无效 |
| 99991621 | get session fail | 开放平台sso请求获取session失败，请检查请求是否带了相应的cookie(lobsession) |
| 99991622 | session invlaid | 开放平台sso请求鉴权失败，请求的session无效 |
| 99991631 | get session fail | lark sso请求获取session失败，请检查请求是否带了相应的cookie(lobsession) |
| 99991632 | session invlaid | lark sso请求 session 校验失败 |
| 99991641 | auth open invalid session | 当前登录态session无效，请重新登录后使用 |
| 99991642 | auth open invalid session mina | 当前登录态session已过期，请重新登录后使用 |
| 99991643 | auth open invalid session sso | open id或refresh token无效，请重新登录，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 99991644 | auth open invalid config | 内部错误，详情请[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 99991645 | auth open invalid user | 当前登录态user无效，请重新登录，仍然出现可以[咨询客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D) |
| 99991661 | Need a token | 请求需要使用token认证；请检查请求Header中是否填了正确的Authorization，注意token值前不要漏了 "Bearer "。详情可参考[API访问凭证概述](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| 99991662 | app not in use | 检查该应用是否处于停用状态 |
| 99991663 | tenant token invalid | tenant_access_token 无效，更新token。详情可参考[API访问凭证概述](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| 99991663 | tenant access token not support | 当前请求不支持tenant_access_token，请检查后重试。 |
| 99991664 | invalid app token | app_access_token 非法，详情可参考[API访问凭证概述](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| 99991665 | invalid tenant code | tenant_access_token 非法，详情可参考[API访问凭证概述](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| 99991668 | token invalid | user_access_token无效，更新token。详情可参考[API访问凭证概述](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| 99991668 | user access token not support | 当前请求不支持user_access_token，请检查后重试 |
| 99991669 | invalid user refresh token | 用户 refresh token 非法 |
| 99991670 | invalid sso token | SSO Access Token非法 |
| 99991671 | Invalid token: must start with t-/u- | 当前请求使用的token格式错误，请检查后重试。详情可参考[API访问凭证概述](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| 99991672 | No permission | 当前请求未申请相关权限。权限的开启需要提交版本发布，通过管理员审核后才能生效，详情参见[开启应用权限方法](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN#e601f785)。对开发测试阶段的应用可以使用“测试版应用”功能，申请权限无需发布版本，等待管理员审核，可直接在测试租户完成，详情参见[教程](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)。 |
| 99991673 | unauthorized app | app状态不可用, 请检查当前租户应用安装状态 |
| 99991674 | user type not support | 当前请求使用的user type无效，请检查后重试 |
| 99991675 | parse user error | 当前请求使用的user_id格式错误，请检查后重试 |
| 99991676 | token no permission | token没有对应的权限，请检查token的权限 |
| 99991677 | token expire | token已过期，请更新token。详情可参考[刷新access_token](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/authen/refresh_access_token) |
| 99991681 | auth fail | 第三方鉴权返回错误, 请参考message进行排查 |
| 99991691 | get session fail | 获取 session 失败，请求未带cookie |
| 99991692 | auth fail | 鉴权失败, 请参考message进行排查 |
| 99991693 | session not exist | 鉴权失败，当前session不存在 |
| 99992351 | these open ids not existed: %v | 部分open_id不存在，请检查后重试 |
| 99992352 | these lark ids not existed: %v | 部分open_id不存在，请检查后重试 |
| 99992353 | these lark ids not existed: %v | 部分open_id不存在，请检查后重试 |
| 99992354 | these ids not existed: %v | message中提示的id不存在，请检查后重试 |
| 99992355 | these lark ids not existed: %v | 当前chat_id不存在，请检查后重试 |
| 99992356 | these open_chat ids not existed: %v | 当前chat_id不存在，请检查后重试 |
| 99992357 | these open_department ids not existed: %v | 当前open_department_id不存在，请检查后重试 |
| 99992358 | these ids not existed: %v | 当前open_department_id不存在，请检查后重试 |
| 99992359 | these user ids not existed: %v | 当前user_id不存在，请检查后重试 |
| 99992360 | these ids not existed: %v | 当前user_id不存在，请检查后重试 |
| 99992361 | open_id cross app | open_id不属于当前app，请检查后重试 |
| 99992363 | these ids not existed: %v | 当前union_id不存在，请检查后重试 |
| 99992364 | these ids not existed: %v | 当前union_id不存在，请检查后重试 |
| 99992364 | user id cross tenant | 不能使用其他租户下的user_id，请检查后重试 |
| 99992370 | these ids not existed: %v | 当前open_app_version_id不存在，请检查后重试 |
| 99992375 | these open_room ids not existed: %v | 当前open_room_id不存在，请检查后重试 |
| 99992376 | these ids not existed: %v | 当前open_room_id不存在，请检查后重试 |
| 99992378 | these open_room ids not existed: %v | 当前open_app_version_id不存在，请检查后重试 |
| 99992379 | these ids not existed: %v | 当前open_department_id不存在，请检查后重试 |
| 99992380 | these ids not existed: %v | 当前open_department_id不存在，请检查后重试 |
| 99992381 | union_id cross tenant | union_id不属于当前tenant。当你使用union_id时，需要注意union_id是否属于当前tenant，请检查后重试 |
| 99992402 | 具体错误信息 | 请求参数缺失或者有误，更多错误信息请参考请求返回的error message |
| 99992500 | these ids not existed: %v | 当前tenant_key不存在，请检查后重试 |
| 99992501 | these ids not existed: %v | 当前tenant_key不存在，请检查后重试 |
| 99993201 | mehod don't support batch | 请确认批量API文档是否包含该API。 |
| 99993202 | the id of subrequest can't be empty | 请参考文档的示例，补充子请求的id |
| 99993203 | the id of subrequest is duplicated | 请确认子请求的id是否重复 |

