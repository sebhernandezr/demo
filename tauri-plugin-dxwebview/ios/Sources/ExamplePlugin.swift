import SwiftRs
import Tauri
import UIKit
import WebKit

class PingArgs: Decodable {
  let value: String?
}

class ExamplePlugin: Plugin {
  private var webView: WKWebView?

  @objc public func ping(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(PingArgs.self)
    invoke.resolve(["value": args.value ?? ""])
  }

  @objc public func createWebview(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(PingArgs.self)
    let urlString = args.value ?? "https://www.google.com"

    DispatchQueue.main.async {
      // Get screen dimensions
      let screenSize = UIScreen.main.bounds.size
      let screenWidth = screenSize.width
      let screenHeight = screenSize.height

      // Create a WKWebView instance
      self.webView = WKWebView(frame: CGRect(x: 0, y: 0, width: screenWidth / 2, height: screenHeight))
      self.webView?.backgroundColor = UIColor.clear
      self.webView?.isOpaque = false

      // Enable JavaScript for the WebView
      self.webView?.configuration.preferences.javaScriptEnabled = true

      // Load the URL in the WebView
      if let url = URL(string: urlString) {
        self.webView?.load(URLRequest(url: url))
      }

      // Add WebView to the main window
      if let window = UIApplication.shared.windows.first {
        window.addSubview(self.webView!)
      }

      invoke.resolve(["value": urlString])
    }
  }

  @objc public func removeWebview(_ invoke: Invoke) throws {
    DispatchQueue.main.async {
      self.webView?.removeFromSuperview()
      self.webView = nil
      invoke.resolve(["value": "WebView removed"])
    }
  }
}

@_cdecl("init_plugin_dxwebview")
func initPlugin() -> Plugin {
  return ExamplePlugin()
}
