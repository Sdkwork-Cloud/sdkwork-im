require "yaml"

package = YAML.load_file(File.join(__dir__, "../pubspec.yaml"))

is_bp = package['name'].start_with?('byteplus')

# Define minimum iOS version
min_ios_version_supported = '11.0'

#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint volc_engine_rtc_flutter.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name             = package["name"]
  s.version          = package["version"]
  s.summary          = 'A new flutter plugin project.'
  s.description      = package["description"]
  s.homepage         = package["homepage"]
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'bytertc' => 'bytertc@bytedance.com' }
  s.source           = { :path => '.' }
  s.source_files = 'Classes/**/*'
  s.public_header_files = 'Classes/**/*.h'
  s.dependency 'Flutter'
  s.module_map = "volc_engine_rtc.modulemap"
  s.ios.deployment_target = '11.0'

  if is_bp
    s.source         = { :git => "https://github.com/byteplus-sdk/byteplus-specs.git", :tag => "#{s.version}" }
  else
    s.source         = { :git => "https://github.com/volcengine/volcengine-specs.git", :tag => "#{s.version}" }
  end

  # Flutter.framework does not contain a i386 slice.
  s.pod_target_xcconfig = {
    'DEFINES_MODULE' => 'YES',
    'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'arm64',
    'EXCLUDED_ARCHS[sdk=iphoneos*]' => 'x86_64',
    'ENABLE_BITCODE' => 'NO'
  }

  # Global configs
  target_volc_api_engine_version = ENV['VOLC_API_ENGINE_VERSION_FOR_RTC']
  rtc_sdk_name = is_bp ? 'BytePlusRTC' : 'VolcEngineRTC'
  real_x_base = is_bp ? 'BytePlusRTC/RealXBase' : 'VolcEngineRTC/RealXBase'
  rtc_default_version = is_bp ? '3.60.103.3330' : '3.60.103.2100'

  # Deps
  s.dependency 'VolcApiEngine', target_volc_api_engine_version || '1.7.0'
  s.dependency 'Flutter'
  s.dependency rtc_sdk_name, rtc_default_version
  # s.dependency real_x_base, rtc_default_version

  # If your plugin requires a privacy manifest, for example if it uses any
  # required reason APIs, update the PrivacyInfo.xcprivacy file to describe your
  # plugin's privacy impact, and then uncomment this line. For more information,
  # see https://developer.apple.com/documentation/bundleresources/privacy_manifest_files
  # s.resource_bundles = {'volc_engine_rtc_flutter_privacy' => ['Resources/PrivacyInfo.xcprivacy']}
end
