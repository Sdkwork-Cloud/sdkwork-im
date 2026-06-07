// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

//
//  ByteRTCEventProtocol.h
//  Pods
//
//  Created by ByteDance on 2025/8/4.
//

@protocol ByteRTCHelperDelegate <NSObject>
@required
/**
 *
 * @type callback
 * @brief 调用 takeLocalSnapshot:callback:{@link #ByteRTCVideo#takeLocalSnapshot:callback:} 截取视频画面时，收到此回调。
 * @param taskId 本地截图任务的编号。和 takeLocalSnapshot:callback:{@link #ByteRTCVideo#takeLocalSnapshot:callback:} 的返回值一致。
 * @param streamIndex 截图的视频流的属性，参看 ByteRTCStreamIndex{@link #ByteRTCStreamIndex}。
 * @param image 截图。你可以保存为文件，或对其进行二次处理。截图失败时，为空。
 * @param errorCode 截图错误码： <br>
 *        - 0: 成功
 *        - -1: 截图错误。生成图片数据失败或 RGBA 编码失败
 *        - -2: 截图错误。流无效。
 *        - -3: 截图错误。截图超时,超时时间 1 秒。
 * @list 高级功能
 * @order 4
 */
- (void) onTakeLocalSnapshotResult:(NSInteger) taskId
                             width:(NSInteger)width
                            height:(NSInteger)height
                          filePath:(NSString * _Nonnull)filePath
                         errorCode:(NSInteger)errorCode;
/**
 *
 * @type callback
 * @brief 调用 takeRemoteSnapshot:callback:{@link #ByteRTCVideo#takeRemoteSnapshot:callback:} 截取视频画面时，收到此回调。
 * @param taskId 远端截图任务的编号。和 takeRemoteSnapshot:callback:{@link #ByteRTCVideo#takeRemoteSnapshot:callback:} 的返回值一致。
 * @param streamKey 截图的视频流，参看 ByteRTCRemoteStreamKey{@link #ByteRTCRemoteStreamKey}。
 * @param image 截图。你可以保存为文件，或对其进行二次处理。截图失败时，为空。
 * @param errorCode 截图错误码： <br>
 *        - 0: 成功
 *        - -1: 截图错误。生成图片数据失败或 RGBA 编码失败
 *        - -2: 截图错误。流无效。
 *        - -3: 截图错误。截图超时,超时时间 1 秒。
 * @list 高级功能
 * @order 5
 */
- (void) onTakeRemoteSnapshotResult:(NSInteger)taskId
                           streamId:(NSString * _Nonnull)streamId
                         streamInfo:(ByteRTCStreamInfo * _Nonnull)streamInfo
                              width:(NSInteger)width
                             height:(NSInteger)height
                           filePath:(NSString * _Nonnull)filePath
                           errorCode:(NSInteger)errorCode;
@end
