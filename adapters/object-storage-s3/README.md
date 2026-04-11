# object-storage-s3

通用 `S3-compatible` 对象存储插件基线。

## 覆盖的插件实例

- `object-storage-aliyun`
- `object-storage-tencent`
- `object-storage-volcengine`
- `object-storage-aws`
- `object-storage-google`
- `object-storage-microsoft`

## 设计约束

- 业务层只依赖 `ObjectStorageProvider`。
- provider 选择由 `ProviderRegistry` 决定。
- URL 签发统一走 `signed_download_url(...)`。
- Google / Microsoft 使用 `s3-gateway` 能力标识。
