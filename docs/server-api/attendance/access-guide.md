---
document_id: '7070902815602884614'
directory_id: '7070770034936053765'
title: 接入指南
full_path: /uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/attendance-development-guidelines
breadcrumb:
- Server API
- Attendance
- Access guide
document_type: GuideDocumentType
updated_at: 2023-08-31T09:15:24Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/attendance-development-guidelines
---

# 接入指南

## 接口能力
考勤 API 提供了丰富的接口开放能力，开发者通过 API 接口，可以获取授权范围内的用户打卡结果、打卡流水记录、考勤组详情、排班详情等信息；也可以创建、修改及删除考勤组、班次等内容。

## API 接入开发流程
**1. 进入[Lark开放平台-开发者后台](https://open.larksuite.com/app/)**

**2. 在开放平台页面，点击“创建应用”**
- 2.1 选择“企业自建应用”
- 2.2 填写应用名称，建议填写“打卡API”
- 2.3 填写应用副标题，建议填写“创建开发者账号”
- 2.4 点击“确定创建”，生成应用

**3. 在开放平台页面，点击已创建的应用**

- 3.1 查看 App ID 和 App Secret
- 3.2 上传一个新的应用图标
- 3.2 点击“安全设置”，设置打卡 API 调用的 IP 白名单

**4. 在开放平台页面，开通打卡应用的读和写权限**
- 4.1 点击“权限管理”，开通“打卡”应用的读写权限

**5. 在开放平台页面，发布自建应用**
- 5.1 点击“应用功能”-“机器人”，启用机器人
- 5.2 点击“版本管理与发布”，发布一个版本
- 5.3 若企业设置了发布审核，需待**企业管理员**审核通过，该应用的开发者账号才可生效

**6. 返回[考勤管理后台](https://www.larksuite.com/attendance/manage/statistics/day)，点击右上角“API 接入”；如果发现无此入口，则已完成接入准备，以下流程可以忽略**

:::html
<md-alert type="error">
如未完成API接入，在访问考勤开放接口时，会返回code:1220004的错误。
</md-alert>
:::

- 6.1 API接入成功示例。

![1.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2aa9d70743572ab68d60501116c97e6b_isKxAjePtt.png?height=602&lazyload=true&width=838)

- 6.2 API未接入完成示例。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a005af3386e08083e28e4ed16a3f1973_Xrl31jGKTl.png?height=1320&lazyload=true&width=1696)



## 事件订阅开发流程
**1. 进入[Lark开放平台-开发者后台](https://open.larksuite.com/app/)**

**2. 在开放平台页面，点击“创建应用”**  （若在 API 接入开发中已创建，则可跳过此步骤）
- 2.1 选择“企业自建应用”
- 2.2 填写应用名称，建议填写“打卡API”
- 2.3 填写应用副标题，建议填写“创建开发者账号”
- 2.4 点击“确定创建”，生成应用


**3. 在开放平台页面，开通打卡应用的读和写权限** （若在 API 接入开发中已开通，则可跳过此步骤）
- 3.1 点击“权限管理”，开通“打卡”应用的读写权限

**4. 在开放平台页面，订阅事件** 
- 4.1 点击“事件订阅”，在“请求网址 URL”处填写回调地址
- 4.2 添加事件，选择考勤，并选择需要的子事件

**5. 在开放平台页面，发布自建应用**
- 5.1 点击“应用功能”-“机器人”，启用机器人（若在 API 接入开发中已启用，则可跳过此步骤）
- 5.2 点击“版本管理与发布”，发布一个版本
- 5.3 若企业设置了发布审核，需待企业管理员审核通过，该应用的开发者账号才可生效


## 名词解释
**employee_id**

雇员 ID，含义同 user_id，详细解释请参见[名词解释](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/terminology)。
