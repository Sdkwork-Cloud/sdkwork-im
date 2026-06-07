// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

//
//  BasicTransformer.h
//  volc_engine_rtc
//
//  Provides basic utility methods for safe JSON/NSDictionary parsing
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

@interface BasicTransformer : NSObject

/**
 * @brief Safely extract string value from dictionary
 * @param source Source dictionary
 * @param key Key to extract
 * @param defaultValue Default value if key not found or invalid type
 * @return Extracted string or default value
 */
+ (NSString *)getString:(nullable NSDictionary *)source 
                 forKey:(NSString *)key 
            withDefault:(NSString *)defaultValue;

/**
 * @brief Safely extract integer value from dictionary
 * @param source Source dictionary
 * @param key Key to extract
 * @param defaultValue Default value if key not found or invalid type
 * @return Extracted integer or default value
 */
+ (NSInteger)getInt:(nullable NSDictionary *)source 
             forKey:(NSString *)key 
        withDefault:(NSInteger)defaultValue;

/**
 * @brief Safely extract double value from dictionary
 * @param source Source dictionary
 * @param key Key to extract
 * @param defaultValue Default value if key not found or invalid type
 * @return Extracted double or default value
 */
+ (double)getDouble:(nullable NSDictionary *)source 
             forKey:(NSString *)key 
        withDefault:(double)defaultValue;

/**
 * @brief Safely extract boolean value from dictionary
 * @param source Source dictionary
 * @param key Key to extract
 * @param defaultValue Default value if key not found or invalid type
 * @return Extracted boolean or default value
 */
+ (BOOL)getBoolean:(nullable NSDictionary *)source 
            forKey:(NSString *)key 
       withDefault:(BOOL)defaultValue;

/**
 * @brief Safely extract float value from dictionary
 * @param source Source dictionary
 * @param key Key to extract
 * @param defaultValue Default value if key not found or invalid type
 * @return Extracted float or default value
 */
+ (float)getFloat:(nullable NSDictionary *)source 
           forKey:(NSString *)key 
      withDefault:(float)defaultValue;

/**
 * @brief Safely extract byte array (NSData) from dictionary
 * @param source Source dictionary
 * @param key Key to extract
 * @param defaultValue Default value if key not found or invalid type
 * @return Extracted NSData or default value
 */
+ (nullable NSData *)getBytes:(nullable NSDictionary *)source 
                       forKey:(NSString *)key 
                  withDefault:(nullable NSData *)defaultValue;

/**
 * @brief Safely extract enum value by index from dictionary
 * @param source Source dictionary
 * @param key Key to extract
 * @param enumValues Array of possible enum values
 * @return Extracted enum value or -1 if invalid
 */
+ (NSInteger)getEnumByIndex:(nullable NSDictionary *)source 
                     forKey:(NSString *)key 
                 enumValues:(NSArray<NSNumber *> *)enumValues;

/**
 * @brief Safely extract array from dictionary
 * @param source Source dictionary
 * @param key Key to extract
 * @return Extracted array or nil
 */
+ (nullable NSArray *)getJSONArray:(nullable NSDictionary *)source 
                            forKey:(NSString *)key;

/**
 * @brief Safely extract dictionary from dictionary
 * @param source Source dictionary
 * @param key Key to extract
 * @return Extracted dictionary or nil
 */
+ (nullable NSDictionary *)getJSONObject:(nullable NSDictionary *)source 
                                  forKey:(NSString *)key;

@end

NS_ASSUME_NONNULL_END
