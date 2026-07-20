---
document_id: '7139727756257984518'
directory_id: '7137610938320961542'
title: 同步三方审批实例
full_path: /home/quickly-develop-three-party-approvals/three-party-approval-instance-synchronization
breadcrumb:
- Home
- Quickly develop three-party approvals
- Synchronous Three-Party Approval Example
document_type: GuideDocumentType
updated_at: 2023-05-17T03:03:39Z
source_url: https://open.larksuite.com/document/home/quickly-develop-three-party-approvals/three-party-approval-instance-synchronization
---

# 同步三方审批实例

在三方审批定义创建完成后，如果用户通过配置的三方系统地址发起审批，你需要将审批流转后生成的审批实例、审批任务、审批抄送数据同步到Lark审批中心。此时你可以调用[三方审批实例同步](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/create)接口实现数据同步功能。

## 操作步骤

1. 在API列表中依次选择**审批** > **三方审批实例** > **同步三方审批实例**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ab5be425b1ea4d54e3a6559c95ea5eeb_IsgdbkucPO.png?lazyload=true&width=2332&height=1728)

2. 单击**请求体**，然后填入三方实例的信息，最后单击右上角**开始调试**。
     
     ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2e36760a2412526dfc30f10de199f2e8_nR73DaDVd2.png?lazyload=true&width=2888&height=1768)
    
    请求体示例如下：
    :::note
    参数说明可参考[三方审批实例同步](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/create)接口。
    :::
   ```
    {
      "approval_code": "84C18825-A3D2-41D0-891F-E7A2424C5D48",
      "instance_id": "3162634147",
      "status": "PENDING",
      "extra": "",
      "links": {
        "pc_link": "https://www.baidu.com/",
        "mobile_link": "https://www.baidu.com/"
      },
      "title": "@i18n@1",
      "form": [{
        "name": "@i18n@2",
        "value": "@i18n@3"
      }],
      "user_id": "16fb9ff3",
      "user_name": "zhangsan",
      "open_id": "123",
      "department_id": "",
      "department_name": "hr",
      "start_time": "1657525395000",
      "update_time": "1657525395000",
      "end_time": 0,
      "update_mode": "REPLACE",
      "task_list": [{
        "task_id": "112253",
        "user_id": "16fb9ff3",
        "links": {
          "pc_link": "https://www.baidu.com/",
          "mobile_link": "https://www.baidu.com/"
        },
        "status": "PENDING",
        "extra": "",
        "title": "external approval test",
        "create_time": "1657525395000",
        "end_time": 0,
        "update_time": "1657525395000",
        "action_context": "123456",
        "action_configs": [{
          "action_type": "APPROVE",
          "action_name": "@i18n@1",
          "is_need_reason": false,
          "is_reason_required": false,
          "is_need_attachment": false

        },
          {
            "action_type": "REJECT",
            "action_name": "@i18n@5"

          }]
      }],
      "cc_list": [{
        "cc_id": "1231243",
        "user_id": "16fb9ff3",
        "open_id": "",
        "links": {
          "pc_link": "https://www.baidu.com/",
          "mobile_link": "https://www.baidu.com/"
        },
        "read_status": "READ",
        "extra": "",
        "title": "XXX",
        "create_time": "1657525395000",
        "update_time": "1657525395000"
      }],
      "i18n_resources": [{
        "locale": "zh-CN",
        "is_default": true,
        "texts": [ {"key":"@i18n@1", "value":"approval test"},
          {"key":"@i18n@2", "value":"day"},
          {"key":"@i18n@3", "value":"2022-07-11"},
          {"key":"@i18n@5", "value":"Cancel"}]
      }]
    }
    ```

3. 接口调用成功后，可以在审批中心中浏览三方系统同步过来的实例、任务、抄送信息，并且可以跳转回三方系统进行更详细的查看和操作，其中实例信息在**已发起**列表，任务信息在**待办**和**已办**列表，抄送信息在**抄送我**列表。
   
   ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/34b18a6bf9f117916bffee54a23e8633_0lrlcVuZQK.png?lazyload=true&width=553&height=570)
