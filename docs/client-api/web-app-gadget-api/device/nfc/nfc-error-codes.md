---
document_id: '7143913324618039301'
directory_id: '6907567266541273090'
title: NFC API 错误码
full_path: /uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- NFC error codes
document_type: GuideDocumentType
updated_at: 2024-03-07T08:40:38Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes
---

# NFC API 错误码
|errno               |errString    |说明   |
|------------------|--------|-----|
|102               |Internal error      |内部错误   |
|1504001           |NFC is not avaliable|手机不支持 NFC |
|1504003             |system NFC switch not opened|手机 NFC 开关未打开|
|1504204             |Tech already connected|NFC 已连接|
|1504205             |Tech has not connected|NFC 未连接 |
|1504206             |NFC tag has not been discoveryed|系统未发现 NFC 标签|
|1504208             |unavailable tech|当前 NFC 标签不支持该tech|
|1504209             |function not support|不支持该 NFC 能力|
|1504211             |data is null |数据传输参数错误|
|1504212             |array buffer is empty|数据传输参数错误|
|1504213             |base64 value is empty|数据传输参数错误|
|1504214             |base64 decode failed|数据传输参数错误|
|1504215             |transceive data failed|数据传输command协议错误|
|1504216             |nfc type is empty|NFC type为空|
|1504217             |nfc service dead|NFC 已断开|
