#import "TcppingPlugin.h"
#if __has_include(<tcpping/tcpping-Swift.h>)
#import <tcpping/tcpping-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "tcpping-Swift.h"
#endif

@implementation TcppingPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftTcppingPlugin registerWithRegistrar:registrar];
}
@end
