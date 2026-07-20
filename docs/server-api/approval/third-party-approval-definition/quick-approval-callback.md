---
document_id: '7122386910857592838'
directory_id: '7122028361538863110'
title: 三方快捷审批回调
full_path: /ukTMukTMukTM/ukjNyYjL5YjM24SO2IjN/quick-approval-callback
breadcrumb:
- Server API
- Approval
- Third-party approval definition
- Quick approval callback
document_type: GuideDocumentType
updated_at: 2023-01-31T12:16:58Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukjNyYjL5YjM24SO2IjN/quick-approval-callback
---

# 三方快捷审批回调

审批人在【待审批】列表中，对审批任务进行【同意】【拒绝】操作时，审批中心会调用审批定义中配置的回调URL通知三方系统。<br>

三方系统收到回调后，进行流程流转，并将最新的任务信息通过审批实例同步接口同步回审批中心。<br>

回调超时时间5s, 超过5s没有返回，卡片会报错，不会更新。

回调失败时（http code != 200），尝试从response body中获取 *message* 字段并展示给用户。

### 接口定义

```json
# 未加密（该代码为示例代码，企业须填写企业对应信息）
curl --location --request POST 'https://www.larksuite.com/approval/openapi/v2/external/instanceOperate' \
--header 'Content-Type: application/json' \
--data-raw '{
  "action_type": "APPROVE",
  "action_context": "12345",
  "user_id": "b85s39b",
  "approval_code": "A3895051-9B16-4ABA-B785-AD2986177BB1",
  "instance_id": "people_1234",
  "task_id": "step1",
  "reason": "ok",
  "attachments": [
      {
          "file_type": "IMAGE",
          "file_name": "1.png",
          "file_size": 12345,
          "url": "https://sf3-cn.larkcdn.com/obj/lark-approval-attachment/image/20200512/413342ae-957f-4c6f-8d06-7dea05875d8b"
      }
  ],
  "token": "aaaaa"
}'

# 加密
curl --location --request POST 'https://www.larksuite.com/approval/openapi/v2/external/instanceOperate' \
--header 'Content-Type: application/json' \
--data-raw '{
    "encrypt": "=sfasdfasfsdafasfaf="
}'
```
```js
# 解密 示例代码

package decrypt

import (
   "crypto/aes"
   "crypto/cipher"
   "crypto/rand"
   "crypto/sha256"
   "encoding/base64"
   "fmt"
   "io"
)

func CBCDecrypter(decryptContent string, keyStr string) ([]byte, error) {
   buf, err := base64.StdEncoding.DecodeString(decryptContent)
   if err != nil {
      return nil, err
   }
   key := sha256.Sum256([]byte(keyStr))
   if len(buf)%aes.BlockSize != 0 {
      return nil, fmt.Errorf("plaintext is not a multiple of the block size")
   }
   block, err := aes.NewCipher(key[:sha256.Size])
   if err != nil {
      return nil, err
   }
   ciphertext := make([]byte, aes.BlockSize+len(buf))
   iv := ciphertext[:aes.BlockSize]
   if _, err := io.ReadFull(rand.Reader, iv); err != nil {
      return nil, err
   }
   mode := cipher.NewCBCDecrypter(block, iv)
   mode.CryptBlocks(ciphertext[aes.BlockSize:], buf)
   ciphertext = ciphertext[32:]

   plain := standardizeDataDe(ciphertext)
   return plain, nil
}

func standardizeDataDe(origData []byte) []byte {
   length := len(origData)
   unpadding := int(origData[length-1])
   if unpadding > length {
      return nil
   }
   return origData[:(length - unpadding)]
}
```



### 接口描述
|参数|类型|必须|说明|
|-|-|-|-|
|action_type|string|是|操作类型<br>APPROVE - 同意<br> REJECT - 拒绝|
|action_context|string|否|操作上下文|
|user_id|string|是|操作人|
|approval_code|string|是|审批定义 code|
|instance_id|string|否|实例 id （列表操作时必填）|
|task_id|string|否|任务 id （列表操作时必填）|
|message_id|int64|否|发送消息卡片返回的 message_id（卡片操作时必填）|
|id|string|否|搜索返回的任务 id（搜索后操作时必填）|
|reason|string|否|原因|
|attachments|list|否|附件|
|&emsp;∟file_type|string|是|附件类型:<br>IMAGE<br> ATTACHMENTS|
|&emsp;∟file_size|int|是|文件大小|
|&emsp;∟file_name|string|是|文件名|
|&emsp;∟url|string|是|附件 cdn url |
|token|string|是|定义配置的 action_callback_token|
|encrypt|string|否|如果定义配置了 action_callback_key ，则上面的参数会进行加密后 放在该字段，业务方需要使用 key 进行解密|

