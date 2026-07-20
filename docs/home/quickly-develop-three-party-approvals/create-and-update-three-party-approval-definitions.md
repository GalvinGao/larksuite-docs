---
document_id: '7139727755097702405'
directory_id: '7137610938320961542'
title: 创建和更新三方审批定义
full_path: /home/quickly-develop-three-party-approvals/create-and-update-three-party-approval-definitions
breadcrumb:
- Home
- Quickly develop three-party approvals
- Create and Update three-party Approval Definitions
document_type: GuideDocumentType
updated_at: 2023-05-17T03:03:35Z
source_url: https://open.larksuite.com/document/home/quickly-develop-three-party-approvals/create-and-update-three-party-approval-definitions
---

# 创建和更新三方审批定义

审批定义是审批的描述，包括审批名称、图标、描述等基础信息。创建好审批定义，用户就可以在审批应用的发起页中看到审批，如果用户点击发起，则会跳转到配置的发起三方系统地址去发起审批。

## 操作步骤

你可以参考以下步骤，调用[创建三方审批定义](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/create)接口，创建三方审批定义。
1. 在API列表中依次选择**审批** > **三方审批定义** > **创建三方审批**。
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c00c611de46c681e64c5b301f17d0551_0QDQQrDr9T.png?lazyload=true&width=2210&height=1664)
2. 单击**权限管理**，查看API权限是否已开通。
    如果尚未开通，你只需勾选未开通的权限，然后单击**批量开通**即可。
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/284b7c272f31753115f44ba3aaf9c751_SV71eK8Wx1.png?lazyload=true&width=2386&height=1462)
3. 单击查询参数，然后选择department_id_type和user_id_type参数的值。
    
    - **department_id_type**：选择部门的ID类型，不填默认为open_department_id。
    - **user_id_type**：选择用户的ID类型，不填默认为open_id。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0ab04a4769a3f5d25ce3fc3eddaa384d_huNRBwUpSZ.png?lazyload=true&width=2368&height=1470)

4. 单击**请求体**， 然后填入三方审批定义信息，最后单击右上角**开始调试**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/87fe3f9439374f3b5fe6675d41c5b874_9VKGkP6N44.png?lazyload=true&width=2886&height=1738)
    
    请求体示例如下：
    :::note
    参数说明可参考[创建三方审批定义](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/create)。
    :::
    ```
    {
        "approval_name": "@i18n@1",
        "approval_code": "permission_test12",
        "group_code": "work_group",
        "group_name": "@i18n@2",
        "external": {
            "create_link_pc": "https://www.feishu.com/",
            "create_link_mobile": "https://www.feishu.com/",
            "support_pc": true,
            "support_mobile": true,
            "support_batch_read": true,
            "action_callback_url": "http://feishu.cn/approval/openapi/operate",
            "action_callback_token": "sdjkljkx9lsadf110",
            "action_callback_key": "gfdqedvsadfgfsd",
            "enable_mark_readed": true,
            "enable_quick_operate": true,
            "key": "",
            "token": ""
        },
        "i18n_resources": [
            {
                "locale": "zh-CN",
                "is_default": true,
                "texts": [
                    {
                        "key": "@i18n@1",
                        "value": "people"
                    },
                    {
                        "key": "@i18n@2",
                        "value": "hr"
                    }
                ]
            }
        ],
        "viewers": [
            {
                "viewer_type": "TENANT"
            }
        ]
    }
    ```
5. 审批定义创建完成后，你可以在Lark审批应用中进行查看和发起审批。
   
   发起审批时，会跳转到配置的三方系统地址去发起审批操作。
   ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2ead32af2bcc50a159a3f0d4dc41199b_xwFcPBeLXl.png?lazyload=true&width=2284&height=1158)

:::note
在当需要更新审批定义时，只需要保证approval_code相同即可。
:::
