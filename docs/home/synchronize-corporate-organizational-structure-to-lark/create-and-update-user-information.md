---
document_id: '7074952334765948934'
directory_id: '7073442394955546629'
title: 创建和更新用户信息
full_path: /home/synchronize-corporate-organizational-structure-to-feishu/create-and-update-user-information
breadcrumb:
- Home
- Synchronize corporate organizational structure to Lark
- Create and update user information
document_type: GuideDocumentType
updated_at: 2023-05-16T08:44:06Z
source_url: https://open.larksuite.com/document/home/synchronize-corporate-organizational-structure-to-feishu/create-and-update-user-information
---

# 创建和更新用户信息

本文将以API调试台工具为例，调用通讯录相关接口创建和更新用户。

## 创建用户

1. 打开[API调试台](https://open.larksuite.com/api-explorer)工具。

2. 在左侧API列表依次选择**通讯录** > **用户** > **创建用户**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/52d3353ec26dfc16c9a80a6a1e0961cb_pARzSCVNYl.png?lazyload=true&width=2298&height=1536)

3. 在请求体中填入`产品经理A`的用户信息及所属产品部的`open_department_id`
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2c2797c494ee7a6670adbbc1be17da7e_u9dVjgC8Xy.png?lazyload=true&width=2774&height=1494)

4. 单击右上角**开始调试**。
    
    调用成功后**调试结果**中会返回，创建成功的用户信息。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/be4bd3ef6807cad31bbf19d9e8fe2c99_E88tBKdkYv.png?lazyload=true&width=2758&height=1544)

5. 重复上述操作，直至所有用户都创建成功。

## 更新用户所有信息

1. 在API调试台左侧的API列表中，依次选择**通讯录** > **用户** > **更新用户所有信息**。

2. 在路径参数中填写需要修改的用户user_id信息。
    
    根据你填写的用户ID类型的不同，你还需要修改查询参数中的**user_id_type**为user_id。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/314ab7c859ec4cbf017a7d606f438078_sgMf0cXtrU.png?lazyload=true&width=1588&height=666)

3. 在请求体中，填入需要修改的部门信息，最后单击右上角**开始调试**按钮。
    
    调用成功后，**调试结果**中会返回，发生变更的部门信息。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/121a5fb4c128b6b30bbead4b96b24e7c_f5xhQGmtWC.png?lazyload=true&width=2756&height=1536)

## 设置部门负责人

1. 在API调试台左侧的API列表中，依次选择**通讯录** > **部门** > **更新部门所有信息**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8994d4ad0eb761214b6871e01242eab4_TAVDewdt7o.png?lazyload=true&width=2098&height=1548)

2. 在请求体中添加`leader_user_id`字段，填入部门负责人的`open_id`。
    
    下图所示为将产品经理A设置为产品部部门负责人示例。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2e3e9258eabbc63c70c1bf4479bc6f14_pShqlFag62.png?lazyload=true&width=2770&height=1550)


