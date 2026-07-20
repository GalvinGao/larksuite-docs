---
document_id: '7139727756258082822'
directory_id: '7137610938320961542'
title: 三方审批实例校验
full_path: /home/quickly-develop-three-party-approvals/three-party-approval-example-verification
breadcrumb:
- Home
- Quickly develop three-party approvals
- Three-Party Approval Example Verification
document_type: GuideDocumentType
updated_at: 2023-05-17T03:03:46Z
source_url: https://open.larksuite.com/document/home/quickly-develop-three-party-approvals/three-party-approval-example-verification
---

# 校验三方审批实例

你可以调用[校验三方审批实例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/check)接口判断Lark审批后台的数据，是否为最新数据。如果审批后台不存在该实例，或者服务端实例更新时间不是最新的，则返回对应实例 ID。
1. 在API列表中依次选择**审批** > **三方审批实例** > **校验三方审批实例**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6a78a04caf63c4c90a19ecc73fbfa3f1_qBpfLHmTOM.png?lazyload=true&width=2516&height=1714)

2. 单击请**求体**，然后在请求体中填入需要校验的实例信息，最后单击右上角**开始调试**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0da41107ff7e8005a6c18804b7ab9820_BxYpopBqmu.png?lazyload=true&width=2876&height=1712)
    
    **请求示例**
    ```
    curl --location --request POST 'https://open.larksuite.com/open-apis/approval/v4/external_instances/check' \
    --header 'Authorization: Bearer t-790b33160a8af2387c93dad2f61e07d8d9f4f2e1' \
    --header 'Content-Type: application/json; charset=utf-8' \
    --data-raw '{
        "instances": [
            {
                "instance_id": "3162634145",
                "update_time": "1657093395000",
                "tasks": [
                    {
                        "task_id": "112253",
                        "update_time": "1638468921000"
                    }
                ]
            }
        ]
    }'
    ```
:::note
如果实例不一致，则会返回不一致的详情，如果一致，则会返回空。
:::
