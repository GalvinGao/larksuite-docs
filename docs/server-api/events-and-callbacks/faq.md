---
document_id: '7382052898484273157'
directory_id: '7262910572679725061'
title: 常见问题
full_path: /uAjLw4CM/ukTMukTMukTM/event-subscription-guide/event-subscriptions/faq
breadcrumb:
- Server API
- Events and callbacks
- FAQ
document_type: GuideDocumentType
updated_at: 2026-04-16T13:25:56Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/event-subscription-guide/event-subscriptions/faq
---

# 常见问题

本文汇总了事件订阅相关的常见问题。

## Q: 如何配置机器人能够接收消息和发送消息
开启机器人对话，需要进行事件配置，同时，需要添加机器人接收消息的事件和配置相应权限，机器人才能接收消息，如果需要机器人拥有发送消息的能力，还需要配置机器人发送消息的 API 能力。以上配置保存并发版后才能开启机器人对话的能力。
1. 进行事件配置：订阅方式，可以选择“使用长连接接收事件”或“将事件发送至开发者服务器”，配置完成后，约1分钟，点击机器人对话页面，可给机器人发送消息

	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fa202c7393e044272ccf35f0ee99ce8b_8eAbFqdoU8.png?height=1388&lazyload=true&width=2914)
    注：若需要机器人能够接收到消息并处理，还需要添加「接收消息」的事件和配置相关权限
2. 添加「接收消息」的事件
    1. 点击添加事件，添加应用身份订阅下的「接收消息」权限
       ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/17bce212fcdfdedd88a6978f7c5cf8b5_HwAsXWh5IV.png?height=1400&lazyload=true&width=2910)
       
       ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fb6f1255cab2e9afab86379de6f2ab77_wRZHKZ6goZ.png?height=1406&lazyload=true&width=2904)
    2. 点击权限名称后出现弹窗，在弹窗中可点击开通权限，当列表显示「已开通」，则机器人开通了对应权限
		![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e8dfcc117315637ea715b4ebd8145125_F2zGAZI8hf.png?height=1404&lazyload=true&width=2896)
    
    	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2d813715ab12c6e53ca93f7a4c12ce2a_6fd2vONTcT.png?height=1382&lazyload=true&width=2900)
3. 配置机器人发送消息的权限：配置机器人发送消息的 API 权限，机器人才能拥有发送消息的能力，发送消息的API为 [发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)

## Q：配置事件订阅请求地址时报错，无法保存如何处理？

你可以按照以下步骤，依次排查配置是否有问题。

1. 设置的请求地址必须是 IPv4 协议的、公网可访问的请求地址（必须以 `http://` 或 `https://`开头）。
	
    例如，使用内网穿透工具生成的请求地址，因工具不稳定或本地企业设置了防火墙，会导致开发平台无法与你配置的请求地址进行连接。

2. 请求地址所在服务器可以成功接收由开放平台发送的、用于校验请求地址合法性的 HTTP POST 请求，且必须在 1 秒内，将接收到的 challenge 值以 JSON 格式原样返回给开放平台。

	如果应用配置了 Encrypt Key，则需要先解密，然后从解密后的内容中提取出 challenge 值，并必须在 1 秒内原样返回该值作为响应。你可以参考本文 **将事件发送至开发者服务器** 章节的操作步骤，依次检查配置是否有问题。

你可以使用以下 curl 命令行工具，测试所配置的请求地址是否可以成功返回 challenge 值。

```bash
# 注意使用以下命令测试时，请确认应用没有配置 Encrypt Key。如果应用配置了 Encrypt Key，那么 HTTP 的请求包体会有加密。

curl -v --location '{YOUR_CALLBACK_URL}' \
--header 'Content-Type: application/json' \
--data '{
    "challenge": "<test code>",
    "type": "url_verification",
    "token": "<your verification token>"
}'
```
其中：

- `{YOUR_CALLBACK_URL}`：替换成你所配置的请求地址，以 `http://` 或 `https://` 开头。
- `<test code>`：替换成测试用的 challenge code，例如 `ajls384kdj1234`。
- `<your verification token>`：替换成 [开发者后台](https://open.larksuite.com/app) > **应用详情页** > **事件与回调** > **加密策略** 页面中的 **Verification Token** 值。

成功返回结果如下图所示。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b2ba36fd3588c069867a8ac5b62144f9_jaLkd92ft2.png?height=732&lazyload=true&maxWidth=600&width=2780)
  

## Q：配置事件订阅请求地址时，提示“请填写有效的 HTTP/HTTPS URL”如何处理？

**问题现象**

如下图所示，配置了事件订阅请求地址后，在保存时提示“请填写有效的 HTTP/HTTPS URL”。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/21d8ed7bdd7db564e613918c452e4555_nObBGnS94b.png?height=1014&lazyload=true&maxWidth=600&width=1758)
    
**排查建议**

:::note
当你在本地配置用于接收事件订阅的服务器时，建议直接把来自开放平台的 HTTP POST 请求打印出来，以此来确认服务器是否接收到了请求，以及请求的具体内容包含哪些。
:::

1. 先使用以下 curl 命令行工具，测试所配置的请求地址是否可以成功返回 challenge 值。

      ```bash
      # 注意使用以下命令测试时，请确认应用没有配置 Encrypt Key。如果应用配置了 Encrypt Key，那么 HTTP 的请求包体会有加密。

      curl -v --location '{YOUR_CALLBACK_URL}' \
      --header 'Content-Type: application/json' \
      --data '{
          "challenge": "<test code>",
          "type": "url_verification",
          "token": "<your verification token>"
      }'
      ```
      其中：

      - `{YOUR_CALLBACK_URL}`：替换成你所配置的请求地址，以 `http://` 或 `https://` 开头。
      - `<test code>`：替换成测试用的 challenge code，例如 `ajls384kdj1234`。
      - `<your verification token>`：替换成 [开发者后台](https://open.larksuite.com/app) > **应用详情页** > **事件与回调** > **加密策略** 页面中的 **Verification Token** 值。
      
      成功返回结果如下图所示。

      ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b2ba36fd3588c069867a8ac5b62144f9_jaLkd92ft2.png?height=732&lazyload=true&maxWidth=600&width=2780)

2. 如果测试命令能返回和请求体中相同的 challenge code，那么：
 	- 检查开放平台的请求是否访问到了请求地址对应的回调服务器，如果没有访问到回调服务器的记录，那么请检查回调服务器是否因配置了防火墙策略，导致请求无法访问。
 	- 检查配置的请求地址是否是公网可访问的。例如，你可以使用其他外部网络设备重新执行步骤 1 中的 curl 命令，测试是否可以正常返回 challenge 值。

3. 如果测试命令不能返回和请求体中相同 challenge code，请先参考[配置订阅方式](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-subscription-configure-/request-url-configuration-case)文档，编写正确的代码，然后重试。
4. 如果应用配置了 Encrypt Key，那么开放平台会推送加密后的 POST 请求。在请求地址对应的回调服务器内，需要按照[事件解密](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-subscription-configure-/encrypt-key-encryption-configuration-case#679e4309)文档，进行解密后再返回消息体中的 challenge code。
 
      ```json
      # 当应用配置了 Encrypt Key，此时开放平台发起的 challenge code 是被加密过的。
      # 需要开发者按照事件解密文档进行解密。

      {
          "encrypt": "ds3da3sj32421lkkld4xxxx"  // 加密字符串
      }
      ```
     
