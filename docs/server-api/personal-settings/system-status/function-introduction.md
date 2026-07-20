---
document_id: '7184305756149268485'
directory_id: '7182701055624822790'
title: 功能介绍
full_path: /uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/overview
breadcrumb:
- Server API
- Personal Settings
- System status
- Function introduction
document_type: GuideDocumentType
updated_at: 2023-01-03T06:15:33Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/overview
---

# 功能介绍
## 一 业务介绍
**1. 谁应该使用这些API?**

系统状态下的API操作的是租户维度的数据，因此使用者应当是**租户系统状态管理者**，而非租户内个人开发者。

**2. 系统状态字段展示**

| 名称         | 描述        |
| --------- | --------------- | -------   | ----------- | --------- |
|`title` | 对应下图序号① |
|`i18n_title` | 对应下图序号①，当同时存在i18n_title、title时，优先展示i18n_title |
|`icon_key` | 对应下图序号② |
|`color` | 对应下图③ |
|`priority` | 数值不在客户端展示，数值越小，客户端上该系统状态展示优先级越高 |
|`sync_setting->is_open_by_default` | 对应下图④ |
|`sync_setting->title` | 对应下图⑤ |
|`sync_setting->i18n_title` | 对应下图⑤，当同时存在i18n_title、title时，优先展示i18n_title |
|`sync_setting->explain` | 对应下图⑥ |
|`sync_setting->i18n_explain` | 对应下图⑥，i18n_explain、explain，优先展示i18n_explain |

![20221102-154822.jpeg](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/db953a79b29dc2e0ad6bcf27f003afbc_58VW85WAyC.jpeg?lazyload=true&width=696&height=439)

![20221102-154826.jpeg](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d713c64473854894527df8524a2039b2_jv1jTmKeMO.jpeg?lazyload=true&width=352&height=592)



## 二 字段枚举
### icon_key
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 15%;">表情</md-th>
      <md-th style="width: 15%;">icon_key</md-th>
      <md-th style="width: 15%;">表情</md-th>
      <md-th style="width: 15%;">icon_key</md-th>
      <md-th style="width: 15%;">表情</md-th>
      <md-th style="width: 15%;">icon_key</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
     <md-tr>
       <md-td>![GeneralDoNotDisturb](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/16794aef3919f3f20d46d0bdf5c89945.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>GeneralDoNotDisturb</md-td>
       <md-td>![GeneralInMeetingBusy](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/f6cd7c731ec50668b488c9abe1608efc.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>GeneralInMeetingBusy</md-td>
       <md-td>![Coffee](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/f6c935685cf07b57fa39473ca6dd0bc7.png?lazyload=true&width=104&height=96)</md-td>
       <md-td>Coffee</md-td>
     </md-tr>
     <md-tr>
       <md-td>![GeneralBusinessTrip](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/e8e1285751c447e158c1db03645a5392.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>GeneralBusinessTrip</md-td>
       <md-td>![GeneralWorkFromHome](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/619c9d5d48dd945dc71922a3e4d46db7.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>GeneralWorkFromHome</md-td>
       <md-td>![StatusEnjoyLife](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/9ca77c1047e0a768ddd00941eb64f97b.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>StatusEnjoyLife</md-td>
     </md-tr>
    <md-tr>
       <md-td>![GeneralTravellingCar](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/26d4dbd2ce407bc241955fc443a3a1fe.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>GeneralTravellingCar</md-td>
       <md-td>![StatusBus](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/290f72459efa88a464220d7c2ebda57d.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>StatusBus</md-td>
       <md-td>![StatusInFlight](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/6ce4bfb3a212e3ec82cd64c208dbb0d3.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>StatusInFlight</md-td>
     </md-tr>
    <md-tr>
       <md-td>![Typing](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/54509a85bf422471d1f6426ea72445de.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>Typing</md-td>
       <md-td>![EatingFood](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/87b0cc5df92bbb51a471294703e65fc9.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>EatingFood</md-td>
       <md-td>![SICK](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/bff2f1050fa6d35b39c64a550b0f02df.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>SICK</md-td>
     </md-tr>
     <md-tr>
       <md-td>![GeneralSun](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/8d4d5d2e59bbdb8869413b0c67e38cd2.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>GeneralSun</md-td>
       <md-td>![GeneralMoonRest](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/d4cf6c0f8875dce0f42853ee624eae32.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>GeneralMoonRest</md-td>
       <md-td>![StatusReading](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/fc49412c59b174f47fa417cb4a70b739.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>StatusReading</md-td>
     </md-tr>
    <md-tr>
       <md-td>![Status_PrivateMessage](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/048f8c167cbc3d276442b05f0e694cfb.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>Status_PrivateMessage</md-td>
       <md-td>![StatusFlashOfInspiration](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/69ac9c6e44ffbbaf3324404611510ccb.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>StatusFlashOfInspiration</md-td>
       <md-td>![StatusFlashOfInspiration](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/6b1ac06bc77bb5c6a57c0e133c30f50c.png?lazyload=true&width=96&height=96)</md-td>
       <md-td>GeneralVacation</md-td>
     </md-tr>
  </md-tbody>
  
</md-table>
::: 
