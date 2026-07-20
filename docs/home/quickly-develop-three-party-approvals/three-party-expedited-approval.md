---
document_id: '7139727755097686021'
directory_id: '7137610938320961542'
title: 三方快捷审批
full_path: /home/quickly-develop-three-party-approvals/three-party-expedited-approval
breadcrumb:
- Home
- Quickly develop three-party approvals
- Three-party Expedited Approval
document_type: GuideDocumentType
updated_at: 2023-05-15T02:36:41Z
source_url: https://open.larksuite.com/document/home/quickly-develop-three-party-approvals/three-party-expedited-approval
---

# 三方快捷审批
审批人可以通过多种方式进行审批操作。

例如：
可以通过卡片中的【同意】【拒绝】按钮进行操作；
:::html
<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/203f0e454911c2299fb9c745994e5cd8_uE0KTkZsfk.png?lazyload=true&width=1473&height=970" style="width:70%"/>
:::

- 在Lark进行审批操作
	
    当审批人点击同意或拒绝后，Lark审批会将该操作以[回调事件](/document/ukTMukTMukTM/ukjNyYjL5YjM24SO2IjN/quick-approval-callback)的方式推送到三方系统中。三方系统在收到该消息后，需要将该审批实例修改为与审批人操作相匹配的状态。并且将最新的审批实例信息通过 [审批实例同步接口](/document/ukTMukTMukTM/uczM3UjL3MzN14yNzcTN) 同步到Lark审批，然后调用[更新审批bot消息](/document/ukTMukTMukTM/uAjNyYjLwYjM24CM2IjN)接口更新卡片状态。



- 跳转到三方系统进行审批操作
	
    当审批人点击查看详情时会跳转到三方系统，
  
  :::html
  <img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f640483f80e3a545d7207fbe8e5f27d2_aXDA3ToWuy.png?lazyload=true&width=1028&height=620" style="width:70%"/>
  :::

	如下是跳转到了三方系统的页面进行审批

    :::html
    <img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/91760fe52a6ca650bfdf6113744d66df_H6FyHzPX5l.png?lazyload=true&width=1640&height=845" style="width:70%"/>
    :::
