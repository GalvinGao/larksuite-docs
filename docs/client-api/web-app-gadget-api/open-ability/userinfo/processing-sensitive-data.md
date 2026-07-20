---
document_id: '6965379543683481606'
directory_id: '6907567266541699074'
title: 敏感数据处理
full_path: /uYjL24iN/ugjMx4COyEjL4ITM
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- UserInfo
- Processing Sensitive Data
document_type: GuideDocumentType
updated_at: 2022-03-11T04:12:38Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugjMx4COyEjL4ITM
---

# 敏感数据处理

## 校验数据合法性
当开发者通过[getUserInfo](/document/uYjL24iN/ucjMx4yNyEjL3ITM)接口请求到敏感数据时，返回的`signature`字段是在小程序服务器端通过如下算法得到：
```
signature = sha1(`${rawData}${base64(session_key)}`)
```
开发者可以在自己的**服务器端**执行同样的算法，来校验数据是否合法。

## 解密敏感数据

1. 对称解密使用的算法为 `AES-128-CBC`，数据采用 `PKCS#5` 填充。
2. 对称解密的目标密文为 `encryptedData`。
3. 对称解密秘钥 `aeskey = Base64_Decode(session_key)`, `aeskey` 长度为16Byte。
4. 对称解密算法初始向量为 `Base64_Decode(iv)`。


## 代码示例
:::note
代码示例中的 `sessionKey` 为会话密钥， 由 [code2session](/document/uYjL24iN/ukjM04SOyQjL5IDN)  返回。
:::

```js
//nodejs语言
const crypto = require("crypto");

sessionKey = '36f764e22be7fc5edb07f56073a4b770';
iv = 'f210af090c830cd5c47e67eeb4f52001';
message = 'Ssy006/cEV4+a2wUWTGvHuXIkU+vX1KCNUkLntKKVXzggx6yfycO/RDkx55A8TvhI5EUGKJ8X4x3NBCypWlkvODaCSDw6uUBhqbDaRS9vH57Dk9nVcw+u2ANGU4PmW/WX2ihze8hDMpofD/pDfZCkEjnvmqkeZ+jPxq351bpDwTUJ+mae92iL9DuHd+xz6WC5cUx9HE8AbxUD18dGUwt2KcNNi9ePvgcvGZBEgZGG2s=';
function decrypt_with_aes() {
    message = Buffer.from(message, 'base64');
    const decipher = crypto.createDecipheriv(
        'aes-128-cbc',
        Buffer.from(sessionKey, 'hex'),
        Buffer.from(iv, 'hex'),
    );
    let decrypted = decipher.update(message);
    decrypted += decipher.final();
    const data = decrypted.toString();
    const json = JSON.parse(data);
    return json;
}
console.log(decrypt_with_aes())
```


```Python
#python3语言
import base64
import json
from Crypto.Cipher import AES

def decrypt_with_aes():
    session_key = '36f764e22be7fc5edb07f56073a4b770';
    iv = "f210af090c830cd5c47e67eeb4f52001";
    message = "Ssy006/cEV4+a2wUWTGvHuXIkU+vX1KCNUkLntKKVXzggx6yfycO/RDkx55A8TvhI5EUGKJ8X4x3NBCypWlkvODaCSDw6uUBhqbDaRS9vH57Dk9nVcw+u2ANGU4PmW/WX2ihze8hDMpofD/pDfZCkEjnvmqkeZ+jPxq351bpDwTUJ+mae92iL9DuHd+xz6WC5cUx9HE8AbxUD18dGUwt2KcNNi9ePvgcvGZBEgZGG2s=";
    session_key = bytes.fromhex(session_key)
    iv = bytes.fromhex(iv)
    message = base64.b64decode(message)
    print(session_key, len(session_key), type(session_key))
    print(len(message))
    decipher = AES.new(session_key, AES.MODE_CBC, iv)
    decodedMessage = decipher.decrypt(message)
    unpad = lambda s: s[0:-s[-1]]
    decodedMessage = unpad(decodedMessage)
    data = json.loads(decodedMessage)
    return data
print(decrypt_with_aes())

```
