---
document_id: '7026663890609143814'
directory_id: '7002892512470630406'
title: 步骤五：响应用户操作
full_path: /home/build-a-beautiful-message-card-in-5-minutes/feedback-on-user-behavior
breadcrumb:
- Home
- Quick intro to message card
- 'Step 5: Respond to user actions'
document_type: GuideDocumentType
updated_at: 2023-05-15T02:36:16Z
source_url: https://open.larksuite.com/document/home/build-a-beautiful-message-card-in-5-minutes/feedback-on-user-behavior
---

# 步骤五：响应用户操作

在用户对消息卡片进行操作之后，Lark会将用户的操作记录推送到消息卡片请求网址中。你可以解析该用户行为，然后更新消息卡片响应用户操作。本文以JavaSDK处理卡片内容为例，介绍如何在服务端处理卡片回调事件。
## 操作步骤
1. 接收用户操作请求。
	
    用户在点击卡片操作时，Lark会向你的 **消息卡片请求网址** 发起 POST 请求，请求体为 JSON 格式。

    请求体示例如下：
    ```JSON
    {
        "open_message_id": "om_bc766xxxbcc3",
        "tenant_key": "10911cb08b989758",
        "open_id": "ou_d72a3e4xxx622e2e",
        "user_id": "4bxxf8",
        "action": {
            "value": {
                "key": "value",
                "key1": "value1"
            },
            "tag": "button"
        },
        "open_chat_id": "oc_43aaa6xxxe6bcd",
        "token": "c-d669d1xxxb14882f"
    }
    ```

    其中：
     * **open_message_id**：表示消息的唯一 ID。
     * **tenant_key**：表示租户 ID，详细概念参考 [通用参数介绍](ssl:ttdoc/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/terminology#495685b5)。
     * **open_id**：表示当前操作用户的 open_id，详细概念参考[用户身份体系介绍](/document/home/user-identity-introduction/introduction)。
     * **user_id**：表示当前操作者的 user_id，详细概念参考[用户身份体系介绍](/document/home/user-identity-introduction/introduction)。
     * **action**：用户进行操作的交互组件，详细介绍可参考 [交互模块](/document/ukTMukTMukTM/uYjNwUjL2YDM14iN2ATN)。
     * **open_chat_id**：表示会话 ID，详细概念参考 [通用参数介绍](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/terminology#301739cd)。
     * **token**：卡片刷新凭证，你可以使用此 token 调用[延时更新消息卡片](/document/ukTMukTMukTM/uMDO1YjLzgTN24yM4UjN)接口，延时更新卡片内容。
       :::note
        此 token 在 30 分钟内最多可更新 2 次消息卡片。
       :::
     
2. 在代码中解析用户操作，并响应更新后的卡片内容。
   * **实时更新**：如果你需要直接反馈用户操作，可以在接受事件的方法中直接返回新的卡片内容。
    
      例如：在以下 [JavaSDK](https://github.com/larksuite/oapi-sdk-java) 的示例代码中，处理用户操作之后，直接 return 了更新后的卡片内容。

      ```Java
      @RestController
      public class CardActionController {

        //1. 注册卡片处理器
        private final CardActionHandler CARD_ACTION_HANDLER = CardActionHandler.newBuilder("v", "e",
            new CardActionHandler.ICardHandler() {
              @Override
              public Object handle(CardAction cardAction) {
                // 1.1 处理卡片行为
                System.out.println(Jsons.DEFAULT.toJson(cardAction));
                System.out.println(cardAction.getRequestId());

                // 1.2 构建响应卡片内容
                MessageCard card = MessageCard.newBuilder()
                    .cardLink(cardURL)
                    .config(config)
                    .header(header)
                    .elements(new MessageCardElement[]{div, note, image, cardAction, hr})
                    .build();
                return card;
              }
            }).build();

        // 2. 注入 ServletAdapter 示例
        @Autowired
        private ServletAdapter servletAdapter;

        //3. 注册服务路由
        @RequestMapping("/webhook/card")
        public void card(HttpServletRequest request, HttpServletResponse response)
            throws Throwable {
          //3.1 回调扩展包卡片行为处理回调
          servletAdapter.handleCardAction(request, response, CARD_ACTION_HANDLER);
        }
      }
      ```

   * **延时更新**：如果你不需要实时更新卡片内容或者处理用户操作需要较长时间，那么你可以先返回`"{}"`字符串，然后在合适的时机调用[延时更新消息卡片](/document/ukTMukTMukTM/uMDO1YjLzgTN24yM4UjN)或[更新应用发送的消息卡片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/patch)接口更新卡片。
   
		例如：在以下 [JavaSDK](https://github.com/larksuite/oapi-sdk-java) 的示例代码中，处理用户操作之后，直接 return 了`null`。
      ```Java
      @RestController
      public class CardActionController {

        //1. 注册卡片处理器
        private final CardActionHandler CARD_ACTION_HANDLER = CardActionHandler.newBuilder("v", "e",
            new CardActionHandler.ICardHandler() {
              @Override
              public Object handle(CardAction cardAction) {
                System.out.println(Jsons.DEFAULT.toJson(cardAction));
                System.out.println(cardAction.getRequestId());
                return null;
              }
            }).build();

        // 2. 注入 ServletAdapter 示例
        @Autowired
        private ServletAdapter servletAdapter;

        //3. 注册服务路由
        @RequestMapping("/webhook/card")
        public void card(HttpServletRequest request, HttpServletResponse response)
            throws Throwable {
          //3.1 回调扩展包卡片行为处理回调
          servletAdapter.handleCardAction(request, response, CARD_ACTION_HANDLER);
        }
      }
      ```
:::note
* 使用延时更新时，你需要在3秒内以`HTTP 200`状态码响应该请求，并将HTTP Body设置为字符串`"{}"`。
* 如果卡片的更新时机不是由消息卡片的交互组件触发（比如用户在企业自建的审批系统内，而不是消息卡片上完成审批操作），可以调用[更新应用发送的消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/patch)接口更新卡片。
:::
  
