package com.plugin.dxwebview

import android.app.Activity
import android.graphics.Color
import android.util.DisplayMetrics
import android.view.Gravity
import android.view.ViewGroup
import android.webkit.WebView
import android.widget.FrameLayout
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class PingArgs {
  var value: String = ""
}

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = Example()

    @Command
    fun ping(invoke: Invoke) {
        val args = invoke.parseArgs(PingArgs::class.java)

        val ret = JSObject()
        ret.put("value", implementation.pong(args.value ?: "default value :("))
        invoke.resolve(ret)
    }

    private var webView: WebView? = null // Keep a reference to the WebView

    @Command
    fun createWebview(invoke: Invoke) {
        // Get screen dimensions
        val displayMetrics = DisplayMetrics()
        activity.windowManager.defaultDisplay.getMetrics(displayMetrics)
        val screenWidth = displayMetrics.widthPixels

        // Create a WebView instance
        webView = WebView(activity)
        webView?.apply {
            setBackgroundColor(Color.TRANSPARENT)

            // Set WebView size to half the screen height
            val layoutParams = FrameLayout.LayoutParams(
                screenWidth / 2,
                ViewGroup.LayoutParams.MATCH_PARENT
            )
            layoutParams.gravity = Gravity.BOTTOM // Position at the bottom of the screen

            // Add WebView to the activity
            activity.addContentView(this, layoutParams)

            // Enable JavaScript for the WebView
            settings.javaScriptEnabled = true

            // Load google.com in the WebView
            val args = invoke.parseArgs(PingArgs::class.java)
            loadUrl(args.value)

            val ret = JSObject()
            ret.put("value", implementation.pong(args.value ?: "default value :("))
            invoke.resolve(ret)
        }
    }

    @Command
    fun removeWebview(invoke: Invoke) {
        webView?.let { webView ->
            // Remove the WebView from its parent
            (webView.parent as? ViewGroup)?.removeView(webView)
            // Destroy the WebView
            webView.destroy()
            // Nullify the reference
            this.webView = null
        }
    }
}
