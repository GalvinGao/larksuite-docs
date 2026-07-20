---
document_id: '6964308654199701509'
directory_id: '7394375283862142981'
title: 事件概述
full_path: /ukTMukTMukTM/uUTNz4SN1MjL1UzM
breadcrumb:
- Server API
- Events and callbacks
- Event subscriptions
- Event Overview
document_type: GuideDocumentType
updated_at: 2026-03-12T06:56:13Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM
---

# 事件概述

通过事件订阅，应用可以及时响应Lark中的变更事件。当事件发生时，开放平台会按照你配置的订阅方式发送事件消息。Lark开放平台支持的事件请参考[事件列表](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-list)。

## 适用场景

下面列举两个简单的适用场景。开发者可以根据实际需求，订阅不同的事件。

- **实时数据处理**
    
    如果应用对数据的实时性要求比较高，希望能够及时地从Lark同步数据的变化，可以进行事件订阅。例如在用户离职时，应用需要第一时间处理离职用户的业务数据，就可以订阅[员工离职](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/events/deleted)事件。
- **快速事件响应**
   
   如果应用需要及时响应用户的操作，可以进行事件订阅。例如当新人入群时，群机器人需要给新人发送一条欢迎消息，就可以订阅[用户进群](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-member-user/events/added)事件。

## 订阅流程

事件订阅的基本流程如下图所示：

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1a85c2b1613abea44935918b7ef83b2d_5onky9xLF6.png?height=172&lazyload=true&maxWidth=750&width=2368)

1. [（可选）配置 Encrypt Key](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-subscription-configure-/configure-encrypt-key)。配置 Encrypt Key 后，开放平台将推送加密后的事件。加密推送能够让用户数据更加安全。
2. [（可选）修改 Verification Token](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-subscription-configure-/modify-verification-token)。创建应用后，开放平台会自动为应用生成一个 Verification Token，用户也可以修改该 Token。开放平台向应用推送的事件中都带有此 Token，应用可以据此 Token 验证推送的事件是否属于该应用。
3. [配置订阅方式](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-subscription-configure-/use-websocket)。配置订阅方式用于接收开放平台向应用推送的事件消息。当应用订阅的事件发生时，开放平台会按照指定方式发送事件消息。
4. [添加事件](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-subscription-configure-/subscription-event-case)。添加应用需要关注的事件。
5. [申请权限](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-security-verification)。要使订阅的事件生效，还需申请所需的权限。申请权限后，请创建应用版本并提交审核，审核通过后，事件订阅才可生效。
6. [接收并处理事件](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-subscription-configure-/encrypt-key-encryption-configuration-case)。当事件发生时，开放平台会向应用推送事件，应用需要接收并处理事件。

## 事件版本

开放平台提供的事件包括 v1.0 和 v2.0 两个版本。v2.0 版本的事件设置更全面、更合理，建议开发者订阅 v2.0 版本的事件。

如果开发者同时订阅了 v1.0 版本和 v2.0 版本的事件，请确保不要重复订阅同一事件。如下图所示，如果订阅了`通讯录变更v1.0` 和`员工离职 v2.0` 事件，当有一个员工离职时，应用会收到两个相同的员工离职事件。


![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5e57e1bf6b7a60963159d9fb6bd0007d_DHzvOuhrhP.png?height=498&lazyload=true&maxWidth=600&width=837)
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c911c2e24ca5ba4b93dcaf22646c6c24_mJtgI8TPtN.png?height=495&lazyload=true&maxWidth=600&width=837)

## 事件结构

不同版本的事件结构不同。添加事件时，从页面上可以看出，添加的是哪个版本的事件。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/bc8ed3a3902a1223142160499100c229_wCeQDdhrQD.png?height=494&lazyload=true&maxWidth=600&width=838)


### v1.0 版本事件结构

下面列举了一个 v1.0 版本的事件示例。
- `ts` 字段表示事件发送的时间，一般近似于事件发生的时间。
- `uuid` 字段是事件的唯一标识。
- `token` 字段即 Verification Token。
- `event` 结构体记录的是事件的详细信息，不同事件的信息不同。其中，通过 `event.type` 字段，可以判断事件类型。

```json
{ 
    "ts": "1502199207.7171419",
    "uuid": "bc447199585340d1f3728d26b1c0297a",
    "token": "41a9425ea7df4536a7623e38fa321bae",
    "type": "event_callback",
    "event": { 
        "app_id": "cli_9c8609450f78d102",
        "chat_id": "oc_26b66a5eb603162b849f91bcd8815b20",
        "operator": {
            "open_id": "ou_2d2c0399b53d06fd195bb393cd1e38f2",
            "user_id": "gfa21d92"
        },
        "tenant_key": "736588c9260f175c",
        "type": "p2p_chat_create",
        "user": {
            "name": "user_name",
            "open_id": "ou_7dede290d6a27698b969a7fd70ca53da",
            "user_id": "gfa21d92"
        }
    }
}
```

### v2.0 版本事件结构

下面列举了一个 v2.0 版本的事件示例。
- `schema` 字段表示事件的版本。v1.0 版本的事件，无此字段。
- `header.event_id` 字段是事件的唯一标识。
- `header.token` 字段即 Verification Token。
- `header.create_time` 字段表示事件发送的时间，一般近似于事件发生的时间。
- `header.event_type` 字段表示事件类型。
- `event` 结构体记录的是事件的详细信息，不同事件的信息不同。

```json
{
    "schema": "2.0",
    "header": { 
        "event_id": "f7984f25108f8137722bb63cee927e66",
        "token": "066zT6pS4QCbgj5Do145GfDbbagCHGgF",
        "create_time": "1603977298000000",
        "event_type": "contact.user_group.created_v3",
        "tenant_key": "xxxxxxx",
        "app_id": "cli_xxxxxxxx",
    },
    "event":{
    }
}
```

## 事件推送

### 推送周期和频次

订阅的事件发生时，Lark将会通过 HTTP POST 请求发送 JSON 格式的事件数据到预先配置的[订阅方式](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-subscription-configure-/request-url-configuration-case)。

应用收到 HTTP POST 请求后，需要在 3 秒内以 HTTP 200 状态码响应该请求。否则Lark开放平台认为本次推送失败，并以 15秒、5分钟、1小时、6小时 的间隔重新推送事件，最多重试 4 次。

从上述描述可以看出，事件重发的最长时间窗口约为 7.1 小时，请检查和处理在 7.1 小时内的重复事件。可以使用如下方式判断事件唯一性：

- 对于 1.0 版本的事件，通过事件结构中的 `uuid` 字段判断事件唯一性。
- 对于 2.0 版本的事件，通过事件结构中的 `event_id` 字段判断事件唯一性。
下面对事件结构进行了详细介绍。

### 事件推送顺序

为了保证用户的事件可用性以及内外部数据变化一致性，对于部分事件，开放平台使用了**有序事件**的形式进行推送。即在用户对前一事件接收成功后，才会推送下一事件。

对于有序事件，用户需要保证相应前后事件的正常消费，避免造成事件的阻塞或收到事件不及时。即当有序事件发生阻塞时，后续投递的事件会直接进入重试队列排队，等前一事件成功后才会开始重试投递队首的排队事件，开始第一次重试。

事件是否有序，则可查看对应事件页面的详细说明。
