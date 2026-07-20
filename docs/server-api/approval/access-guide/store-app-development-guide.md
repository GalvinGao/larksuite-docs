---
document_id: '6967331158355673094'
directory_id: '7186908899420831749'
title: 商店应用开发指南
full_path: /ukTMukTMukTM/ukzNyYjL5cjM24SO3IjN
breadcrumb:
- Server API
- Approval
- Access guide
- Store app development guide
document_type: GuideDocumentType
updated_at: 2023-01-31T12:16:25Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukzNyYjL5cjM24SO3IjN
---

# 商店应用开发指南

## 使用对象：

Lark第三方企业应用的服务商

## 接入价值：

针对应用服务商开发的第三方企业应用，当应用内涉及审批工作流程时（例如：打卡应用，当员工忘记打卡时，需要发起补卡审批），无需在应用内搭建审批流程相关功能，直接调用审批 API 接口，即可实现应用内的审批工作流，从而降低开发成本，提升用户审批体验。

## 接入流程：
1. 需要应用服务商在Lark开放平台已创建应用。应用创建可参考：[应用创建文档](/document/uMzNwEjLzcDMx4yM3ATM/uUzNwEjL1cDMx4SN3ATM)


**![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f744cc80a15c35c0ad9b71962b0b18de_AYl7cyc8wS.png?lazyload=true&width=1640&height=519)**

2. 指定在特定时间通过 API 创建审批流程（例如：打卡应用，指定当企业开通打卡时，打卡应用通过审批开放的 API 创建补卡审批流程）。API 创建审批流程参考：[创建审批定义](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create)

**![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ba58c4463805ea90615654d05a97d744_MKPeKSj0sc.png?lazyload=true&width=1538&height=448)**

3. 当员工在第三方企业应用或审批的发起页面发起审批时，第三方企业应用通过 API 接口创建审批实例。API 创建审批实例参考：[创建审批实例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/create)

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/06f1b4e0320bd7b1d31e25585e4e2fef_L7P3N91j3m.png?lazyload=true&width=1509&height=1293)

4. 审批人在审批应用中完成审批，审批应用将审批结果同步到第三方企业应用中（例如：审批人在审批应用中完成补卡审批，审批结果返回打卡应用，打卡应用将员工缺卡状态修改为正常）。审批状态同步参考：[订阅审批事件](/document/ukTMukTMukTM/ucDOyUjL3gjM14yN4ITN)

## 第三方企业应用接入审批用到以下接口：

1. [创建审批定义](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create)
2. [创建审批实例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/create)
3. [订阅审批事件](/document/ukTMukTMukTM/ucDOyUjL3gjM14yN4ITN)
